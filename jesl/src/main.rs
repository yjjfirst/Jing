mod esl;
mod event;
mod cmd;

use std::thread;
use std::collections::HashMap;
use std::io;
use crossbeam_channel::{bounded};

use esl::{ Esl, enable_event, enable_cdr };
use cmd::{ Cmd };
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
    let (cmd_s, cmd_r) = bounded::<Cmd>(1);

    let mut esl = Esl::new("127.0.0.1".to_string(),
        "8021".to_string(),
        "ClueCon".to_string(), cmd_r);


    thread::spawn(move || {
        esl
            .start(handle_event)
            .expect("Error connect to FreeSwitch");

    });

    loop {
        let mut input = String::new();        

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim() == "enable event" {
            enable_event(&cmd_s);
        } else if input.trim() == "enable cdr" {
            enable_cdr(&cmd_s);
        }
    }
}
