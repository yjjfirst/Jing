mod esl;
mod event;
mod cmd;

use esl::{Esl};

fn main() {
    let mut esl = Esl::new(
        "127.0.0.1".to_string(),
        "8021".to_string(),
        "ClueCon".to_string());
    let receiver = esl
        .start()
        .expect("Error connect to FreeSwitch");

    while let Ok(event) = receiver.recv() {
        //println!("Receive Message: {:?}", event);
        esl.handle_event(event);
    }
}
