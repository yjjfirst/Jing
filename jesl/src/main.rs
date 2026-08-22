mod esl;
mod event;
mod cmd;

use std::collections::HashMap;

use esl::{ Esl };
use event::{ Event,Request,Reply };

pub fn handle_request(_esl: &mut Esl, _req: Request) {
}

fn handle_reply(_esl: &mut Esl,reply: Reply) {
    match reply {
        Reply::Command { status, text } => {
            println!("{} {}", status, text);
        }
    }
}

pub fn handle_plain(_esl: &mut Esl, _content: HashMap<String, String>) {

}

pub fn handle_event(esl: &mut Esl, event: Event) {
    match event {
        Event::Request(request, _content) => {
            handle_request(esl, request);
        }
        Event::Reply(reply, _content) => {
            handle_reply(esl, reply);
        }
        Event::EventPlain(_plain, content) => {
            handle_plain(esl, content);
        }
    }
}

fn main() {
    let mut esl = Esl::new("127.0.0.1".to_string(),
        "8021".to_string(),
        "ClueCon".to_string());

    esl
        .start(handle_event)
        .expect("Error connect to FreeSwitch");
}
