
use iced::{Subscription, subscription};



use super::{response::LspResponse, transport::MessageReciever};

pub enum State {
    Connected(MessageReciever),
    Disconnected,
}


pub fn connect(client_id: usize, reciever: MessageReciever) -> Subscription<Event> {
    subscription::unfold(client_id, State::Connected(reciever), move |state| {
        recieve_lsp_messages(state)
    })
}

async fn recieve_lsp_messages(state: State) -> (Event, State) {
    match state {
        State::Connected(rx) => {
            let response = rx.wait_for_message().await;
            match response {
                Ok(value) => (Event::Response(value.clone()), State::Connected(rx)),
                Err(_e) => {
                    (Event::Disconnected, State::Disconnected)
                },
            }
        },
        State::Disconnected => {
            iced::futures::future::pending().await
        },
    }
}

pub enum Event {
    Response(LspResponse),
    Disconnected
}