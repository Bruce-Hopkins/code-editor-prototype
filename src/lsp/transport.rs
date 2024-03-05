use std::{fs, path::Path};

use futures::AsyncRead;
use jsonrpc_lite::{JsonRpc, Params};
use lsp::{
    notification::{DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, DidSaveTextDocument, Initialized, Notification}, request::Request, DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams, InitializeResult, InitializedParams, TextDocumentContentChangeEvent, TextDocumentIdentifier, TextDocumentItem, Url, VersionedTextDocumentIdentifier
};
use lsp_types as lsp;
use lsp_types::request::Initialize;
use serde::Serialize;
use serde_json::Value;
use smol::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt},
    process::{ChildStdin, ChildStdout}, channel::{Sender, Receiver}, Task,
};
use smol::{
    io::{BufReader, BufWriter},
    process::ChildStderr,
};

use crate::core::document_change::DocumentChange;

use super::{client::file_path, error::{LspClientError, LspClientResult}, response::LspResponse};
#[derive(Debug)]
pub enum Input {
    Error(String),
    Message(String),
}

async fn process_reader<T>(reader: &mut BufReader<T>, sender: &Sender<Input>, message: impl Fn(String) -> Input) 
    where T: AsyncRead + Unpin + Send {
    let result = read_message(reader).await;
    if !result.is_empty() {
        if let Err(_e) = sender.send(message(result)).await {
            eprintln!("Failed to send a message. Channel must have closed.");
        }
    }

}
pub struct TransortResult {
    pub receiver: Receiver<Input>,
    pub sender: Sender<JsonRpc>,
    pub writing_task: Task<()>,
    pub reading_task: Task<()>
}
pub trait SenderState {}
#[derive(Clone)]
pub struct InitializedSender;

#[derive(Clone)]
pub struct UninitializedSender;

impl SenderState for InitializedSender {}
impl SenderState for UninitializedSender {}

#[derive(Clone)]
pub struct MessageSender<S>
where S: SenderState{
    pub sender: Sender<JsonRpc>,
    pub state: S
}

impl From<Sender<JsonRpc>> for MessageSender<UninitializedSender> {
    fn from(value: Sender<JsonRpc>) -> Self {
        Self {
            sender: value,
            state: UninitializedSender
        }
    }
}

impl <S> MessageSender <S> 
where S: SenderState {
    pub async fn send(self, msg: JsonRpc) {
        self.sender.send(msg).await.unwrap();
    }

        /**
     * Sends an LSP notification
        */
    async fn send_notification<T>(self, method: &str, params: T)
    where
        T: Serialize,
        S: SenderState
    {
        let msg = JsonRpc::notification_with_params(
            method,
            Params::from(serde_json::to_value(params).unwrap()),
        );
        self.send(msg).await
    }

    /**
     * Sends an LSP request
        * 
        * This method should not be used alone. It should only be used with the `Client` method with the same name.
        */
    async fn send_request<T>(self, id: usize, method: &str, params: T)
    where
        T: Serialize,
        S: SenderState
    {
        let msg = JsonRpc::request_with_params(
            id.to_string(),
            method,
            Params::from(serde_json::to_value(params).unwrap()),
        );
        self.send(msg).await
    }
}

impl MessageSender<UninitializedSender> {
    pub fn init(self) -> MessageSender<InitializedSender>{
        MessageSender { sender: self.sender, state: InitializedSender }
    }

    pub async fn initialize(self, init_params: lsp::InitializeParams) {
        self.send_request(1, Initialize::METHOD, init_params)
        .await    
    }
}

impl MessageSender<InitializedSender> {
    pub async fn has_initialized(self) {
        self.send_notification(Initialized::METHOD, InitializedParams {})
        .await;
    }

    pub async fn open_document(self, path: String){
        // Open a document
        let rust_path = Path::new(&path);
        let file = fs::read_to_string(rust_path).unwrap();

        let url = Url::parse(&file_path(&path)).unwrap();
        let params = DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: url,
                language_id: "rust".to_string(),
                version: 1,
                text: file,
            },
        };

        self.send_notification(DidOpenTextDocument::METHOD, params)
        .await;
    }

    pub async fn closed_document(self, path: String) {
        let url = Url::parse(&file_path(&path)).unwrap();
        let params = DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier {
                uri: url,
            }
        };

        self.send_notification(DidCloseTextDocument::METHOD, params)
        .await
    }


    pub async fn doc_changed(self, changes: DocumentChange) {
        let _event = TextDocumentContentChangeEvent {
            range: Some(changes.range.clone().into()),
            range_length: Some(1),
            text: changes.text.clone(),
        };
        let params = DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: Url::parse(&file_path(&changes.file)).unwrap(),
                version: 2,
            },
            content_changes: vec![],
        };
        self.send_notification(DidChangeTextDocument::METHOD, params)
        .await
    }

    pub async fn did_save(self, path: String) {
        let params = DidSaveTextDocumentParams {
            text: None,
            text_document: TextDocumentIdentifier {
                uri: Url::parse(&file_path(&path)).unwrap(),
            },
        };
        self.send_notification(DidSaveTextDocument::METHOD, params)
        .await
    }

}

#[derive(Clone)]
pub struct MessageReciever(Receiver<Input>);

impl From<Receiver<Input>> for MessageReciever {
    fn from(value:  Receiver<Input>) -> Self {
        Self(value)
    }
}

impl MessageReciever {
    /**
     * Receives a message from the LSP. 
     * 
     * If it fails to recieve a message, it will print an error and return early.
     * If it succeeds, it passes into the callback the value recieved.
     */
    async fn recv(&self) -> LspClientResult<Input> {
        let message = self.0.recv().await;
        match message {
            Ok(value) => Ok(value),
            Err(_e) => {

                let message = "Failed to send a recieve message. Channel must have closed.".to_string();
                Err(LspClientError::ChannelClosed(message))
            },
        }
    }

    pub fn input(self) -> Receiver<Input> {
        self.0
    }

    /**
     * Waits for the message and returns it. 
     * 
     * Should be used within a infinite loop to recieve every message.
     */
    pub async fn wait_for_message(&self) -> LspClientResult<LspResponse> {
        let message = self.recv().await?;
        match message {
            Input::Error(err) => Ok(LspResponse::ErrorMessage(err)),
            Input::Message(value) =>{
                let json: Result<Value, serde_json::Error> = serde_json::from_str(&value);

                if !value.is_empty() {
                    if let Ok(json) = json {
                        return Ok(self.get_response_from_message(&json))
                    }
                    return Ok(LspResponse::UnknownMessage)
                }
                Ok(LspResponse::NoMessage)
            }
        }
    }

    fn get_response_from_message(&self, json: &Value) -> LspResponse {
        if let Some(value) = json.get("id") {
            // The message is a request
            //
            let id = if let Some(value) = value.as_str() {
                value.to_string()
            } else if let Some(value) = value.as_u64() {
                value.to_string()
            } else {
                panic!("Id is something unexpected.")
            };
            if id.as_str() == "1" {
                return LspResponse::Initialized
            }
            return LspResponse::NoMessage
        } else if let Some(value) = json.get("method") {

            // The message is a notification.
            let method = value.as_str().unwrap();

            return LspResponse::from_response(method, json)
        }
        panic!("What is this?")
    }

}

async fn recieve_messages(sender: Sender<Input>, reader: BufReader<ChildStdout>, err_reader: BufReader<ChildStderr>) {
    let mut reader = reader;
    let mut err_reader = err_reader;
    loop {
        smol::future::race(
            process_reader(&mut reader, &sender, |result| {Input::Message(result)} ), 
            process_reader(&mut err_reader, &sender, |result| {Input::Error(result)} )
        ).await;
    }
}

async fn write_messages(writer: BufWriter<ChildStdin>, rx: Receiver<JsonRpc>) {
    let mut writer = writer;
    loop {
        let result = rx.recv().await;
        if let Ok(value) = result {
            send(&mut writer, value).await;
        }
    }
}

/**
 * Starts the transport by spawning to async tasks which will listen to errors and inputs from the lsp.
    */
pub fn start_transport(
    reader: BufReader<ChildStdout>,
    writer: BufWriter<ChildStdin>,
    err_reader: BufReader<ChildStderr>,
) -> TransortResult {
    // Receiving messages from the LSP channel
    let (s, receiver) = smol::channel::unbounded::<Input>();
    let reading_task = smol::spawn(recieve_messages(s, reader, err_reader));

    let (sender, rx) = smol::channel::unbounded::<JsonRpc>();
    let writing_task = smol::spawn(write_messages(writer, rx));

    TransortResult {
        reading_task,
        writing_task,
        receiver,
        sender
    }
}

pub async fn initialize(
    reader: &mut BufReader<ChildStdout>,
    writer: &mut BufWriter<ChildStdin>,
    stderr: &mut BufReader<ChildStderr>,
    init_params: lsp::InitializeParams,
) -> LspClientResult<InitializeResult> {
    let init_params = Params::from(serde_json::to_value(init_params).unwrap());

    let msg = JsonRpc::request_with_params(1, Initialize::METHOD, init_params);
    send(writer, msg).await;

    let error_message = recieve_err(stderr);
    let result_message = recieve_input(reader);

    let result = smol::future::race(error_message, result_message).await;

    match result {
        Input::Error(err) => Err(LspClientError::FailedInitiation(err)),
        Input::Message(value) => {
            let value: Value = serde_json::from_str(&value).unwrap();
            let value = value.get("result").unwrap().clone();
            let result: InitializeResult = serde_json::from_value(value).unwrap();
            Ok(result)
        }
    }
}

/**
 * A generic send method for both notifications and requests
    */
async fn send<T>(writer: &mut BufWriter<ChildStdin>, value: T)
where
    T: Serialize,
{
    let value = serde_json::to_string(&value).unwrap();
    let value = format!("Content-Length: {}\r\n\r\n{}", value.len(), value);

    let _ = writer.write_all(value.as_bytes()).await;
    let _ = writer.flush().await;
}


async fn recieve_input<T>(reader: &mut BufReader<T>) -> Input 
where 
T: smol::io::AsyncRead + Unpin {
    let response = read_message(reader).await;
    Input::Message(response)
}

async fn recieve_err<T>(reader: &mut BufReader<T>) -> Input
where 
    T: smol::io::AsyncRead + Unpin
{
    let response = read_message(reader).await;
    Input::Error(response)
}

async fn read_message<T>(reader: &mut BufReader<T>) -> String
where
    T: smol::io::AsyncRead + Unpin
{

    let content_length = get_content_length(reader).await;
    match content_length {
        0 => String::new(),
        content_length => {
            let mut content = vec![0; content_length];
            reader.read_exact(&mut content).await.unwrap();
            let result = std::str::from_utf8(&content).unwrap();
            result.to_owned()
        },

    }

}

async fn get_content_length<T>(reader: &mut BufReader<T>) -> usize 
where 
    T: smol::io::AsyncRead + Unpin { 
let mut buffer = String::new();
    let mut content_length = 0;
    loop {
        if buffer == "\r\n" {break}
        
        buffer.clear();
        if reader.read_line(&mut buffer).await.unwrap() == 0 {
            return 0
        };
        let buffer = buffer.trim();
        let parts = buffer.split_once(": ");

        if let Some(("Content-Length", value)) = parts {
           content_length = value.parse().unwrap()
        }
    }
    content_length
}

