use std::{
    fs, io::Error, mem, path::{Path, PathBuf}, process::Stdio
};



use lsp::WorkspaceFolder;
use lsp_types as lsp;


use serde_json::json;
use smol::{
    io::{BufReader, BufWriter},
    process::{Child, Command},
    Task,
};

use super::{
    error::{LspClientError, LspClientResult}, transport::{start_transport, InitializedSender, MessageReciever, MessageSender, UninitializedSender}
};

#[derive(Clone, Default)]
pub enum LspClient {
    Initialized(MessageSender<InitializedSender>),
    Uninitialized(MessageSender<UninitializedSender>),

    #[default]
    None
}

impl LspClient {
    fn init(self) -> LspClient {
        if let LspClient::Uninitialized(value) = self {
            LspClient::Initialized(value.init())
        }
        else {
            self
        }
    }

    fn as_initialized(&self) -> Option<MessageSender<InitializedSender>> {
        if let LspClient::Initialized(value) = self {
            return Some(value.clone())
        }
        None
    }

    fn as_uninitialized(&self) -> Option<MessageSender<UninitializedSender>> {
        if let LspClient::Uninitialized(value) = self {
            return Some(value.clone())
        }
        None
    }
    
}


struct Tasks {
    _reciever_task: Task<()>,
    _writer_task: Task<()>
}

pub struct LspConnection {
    _process: Child,
    receiver: MessageReciever,
    sender: LspClient,
    _tasks: Tasks,
    file_name:  String,
    file_path: String
}

impl LspConnection {
    pub fn new(file: &Path) -> LspClientResult<Self> {

        let process: Result<smol::process::Child, Error> = Command::new("rust-analyzer")
            // .env("RA_LOG", "info")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn();
        let mut process = match process {
            Ok(process) => process,
            Err(e) => {
                return Err(LspClientError::ProcessFailure(format!(
                    "Failed to start the child process: {}",
                    e
                )))
            }
        };

        let reader = BufReader::new(process.stdout.take().expect("Failed to open stdout"));
        let writer = BufWriter::new(process.stdin.take().expect("Failed to open stdin"));
        let stderr = BufReader::new(process.stderr.take().expect("Failed to open stderr"));

        let file_path = match file.to_str() {
            Some(value) => value,
            None => return Err(LspClientError::FailedInitiation("Invalid path".to_owned()))
        };

        let filename = file.file_name().unwrap().to_str().unwrap().to_owned();
        // 1, change the initialize method of the transport to NOT be static.
        // 2, Create a seprate client that is only for the start state, we won't pass the transport
        // other information there.
        let transport = start_transport(reader, writer, stderr);
        Ok(Self {
            _process: process,
            file_name: filename,
            file_path: file_path.to_owned(),
            sender: LspClient::Uninitialized(MessageSender::from(transport.sender)),
            receiver: MessageReciever::from(transport.receiver),
            _tasks: Tasks {
                _reciever_task: transport.reading_task,
                _writer_task: transport.writing_task
            }
        })
    }

    pub fn new_receiver(&self) -> MessageReciever {
        self.receiver.clone()
    }

    pub fn new_sender(&self) -> LspClient {
        self.sender.clone()
    }

    pub fn has_initialize_client(&mut self) {
        let sender = mem::take(&mut self.sender);
        self.sender = sender.init();
    }

    pub fn init_params(&self) -> lsp::InitializeParams  {
        init_params(self.file_path.clone(), self.file_name.clone())
    }

    pub fn as_initialized(&self) -> Option<MessageSender<InitializedSender>> {
        self.sender.as_initialized()
    }

    pub fn as_uninitialized(&self) -> Option<MessageSender<UninitializedSender>> {
        self.sender.as_uninitialized()
    }

}

pub fn file_path(relative_path: &str) -> String {
    let path = PathBuf::from(relative_path);
    let absolute_path = fs::canonicalize(path).unwrap();

    format!("file://{}", absolute_path.to_str().unwrap())
} 


fn init_params(file: String, name: String) -> lsp::InitializeParams {
    let workspace_url = file_path(&file);

    let workspace = WorkspaceFolder {
        uri: lsp::Url::parse(&workspace_url).unwrap(),
        name,
    };

    let options = Some(json!({
      "server": {
          "extraEnv": { "RUSTUP_TOOLCHAIN": "stable" }
      },
      "trace": {
          "server": "verbose"
      },
      "cargo": {
        "buildScripts": {
          "enable": true,
        },
      },
      "procMacro": {
        "enable": true,
      }
    }));
    lsp::InitializeParams {
        process_id: Some(std::process::id()),
        root_uri: Some(lsp::Url::parse(&workspace_url).unwrap()),
        initialization_options: options,
        capabilities: lsp::ClientCapabilities {
            workspace: Some(lsp::WorkspaceClientCapabilities {
                configuration: Some(true),
                did_change_configuration: Some(lsp::DynamicRegistrationClientCapabilities {
                    dynamic_registration: Some(false),
                }),
                workspace_folders: Some(true),
                apply_edit: Some(true),
                symbol: Some(lsp::WorkspaceSymbolClientCapabilities {
                    dynamic_registration: Some(false),
                    ..Default::default()
                }),
                execute_command: Some(lsp::DynamicRegistrationClientCapabilities {
                    dynamic_registration: Some(false),
                }),
                inlay_hint: Some(lsp::InlayHintWorkspaceClientCapabilities {
                    refresh_support: Some(false),
                }),
                workspace_edit: Some(lsp::WorkspaceEditClientCapabilities {
                    document_changes: Some(true),
                    resource_operations: Some(vec![
                        lsp::ResourceOperationKind::Create,
                        lsp::ResourceOperationKind::Rename,
                        lsp::ResourceOperationKind::Delete,
                    ]),
                    failure_handling: Some(lsp::FailureHandlingKind::Abort),
                    normalizes_line_endings: Some(false),
                    change_annotation_support: None,
                }),
                did_change_watched_files: Some(lsp::DidChangeWatchedFilesClientCapabilities {
                    dynamic_registration: Some(true),
                    relative_pattern_support: Some(false),
                }),
                ..Default::default()
            }),
            text_document: Some(lsp::TextDocumentClientCapabilities {
                completion: Some(lsp::CompletionClientCapabilities {
                    completion_item: Some(lsp::CompletionItemCapability {
                        snippet_support: Some(true),
                        resolve_support: Some(lsp::CompletionItemCapabilityResolveSupport {
                            properties: vec![
                                String::from("documentation"),
                                String::from("detail"),
                                String::from("additionalTextEdits"),
                            ],
                        }),
                        insert_replace_support: Some(true),
                        deprecated_support: Some(true),
                        tag_support: Some(lsp::TagSupport {
                            value_set: vec![lsp::CompletionItemTag::DEPRECATED],
                        }),
                        ..Default::default()
                    }),
                    completion_item_kind: Some(lsp::CompletionItemKindCapability {
                        ..Default::default()
                    }),
                    context_support: None, // additional context information Some(true)
                    ..Default::default()
                }),
                hover: Some(lsp::HoverClientCapabilities {
                    // if not specified, rust-analyzer returns plaintext marked as markdown but
                    // badly formatted.
                    content_format: Some(vec![lsp::MarkupKind::Markdown]),
                    ..Default::default()
                }),
                signature_help: Some(lsp::SignatureHelpClientCapabilities {
                    signature_information: Some(lsp::SignatureInformationSettings {
                        documentation_format: Some(vec![lsp::MarkupKind::Markdown]),
                        parameter_information: Some(lsp::ParameterInformationSettings {
                            label_offset_support: Some(true),
                        }),
                        active_parameter_support: Some(true),
                    }),
                    ..Default::default()
                }),
                rename: Some(lsp::RenameClientCapabilities {
                    dynamic_registration: Some(false),
                    prepare_support: Some(true),
                    prepare_support_default_behavior: None,
                    honors_change_annotations: Some(false),
                }),
                code_action: Some(lsp::CodeActionClientCapabilities {
                    code_action_literal_support: Some(lsp::CodeActionLiteralSupport {
                        code_action_kind: lsp::CodeActionKindLiteralSupport {
                            value_set: [
                                lsp::CodeActionKind::EMPTY,
                                lsp::CodeActionKind::QUICKFIX,
                                lsp::CodeActionKind::REFACTOR,
                                lsp::CodeActionKind::REFACTOR_EXTRACT,
                                lsp::CodeActionKind::REFACTOR_INLINE,
                                lsp::CodeActionKind::REFACTOR_REWRITE,
                                lsp::CodeActionKind::SOURCE,
                                lsp::CodeActionKind::SOURCE_ORGANIZE_IMPORTS,
                            ]
                            .iter()
                            .map(|kind| kind.as_str().to_string())
                            .collect(),
                        },
                    }),
                    is_preferred_support: Some(true),
                    disabled_support: Some(true),
                    data_support: Some(true),
                    resolve_support: Some(lsp::CodeActionCapabilityResolveSupport {
                        properties: vec!["edit".to_owned(), "command".to_owned()],
                    }),
                    ..Default::default()
                }),
                publish_diagnostics: Some(lsp::PublishDiagnosticsClientCapabilities {
                    version_support: Some(true),
                    data_support: Some(true),
                    ..Default::default()
                }),
                inlay_hint: Some(lsp::InlayHintClientCapabilities {
                    dynamic_registration: Some(false),
                    resolve_support: None,
                }),
                ..Default::default()
            }),
            window: Some(lsp::WindowClientCapabilities {
                work_done_progress: Some(true),
                ..Default::default()
            }),
            general: Some(lsp::GeneralClientCapabilities {
                position_encodings: Some(vec![
                    lsp::PositionEncodingKind::UTF32,
                    lsp::PositionEncodingKind::UTF8,
                    lsp::PositionEncodingKind::UTF16,
                ]),
                ..Default::default()
            }),
            ..Default::default()
        },
        trace: Some(lsp_types::TraceValue::Verbose),
        client_info: Some(lsp::ClientInfo {
            name: String::from("my-editor"),
            version: Some(String::from("1.0.0")),
        }),
        root_path: None,
        workspace_folders: Some(vec![workspace]),
        locale: None,
    }
}