mod statis;

use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::io;
use std::io::{BufRead};
use crossbeam_channel::{bounded, unbounded, Sender, Receiver, select, tick};

use jing_lib::esl::esl::{ Esl, event, filter};
use jing_lib::esl::cmd::{ Cmd };
use jing_lib::esl::event::{ Event,Request,Reply };
use jing_lib::firewall::{iptables, self};

pub fn handle_request(req: Request) {
    println!("{:?}", req)
}

fn handle_reply(cmd_s: &Sender<Cmd>, reply: Reply) {
    match reply {
        Reply::Command { text, status } => {
            if text == "accepted" {
                println!("Login ESL successfully");
                event(cmd_s,
                    "CUSTOM",
                    Some("sofia::register_failure"));
                filter(cmd_s, "CUSTOM");
            } else {
                println!("{} {}", status, text);
            }
        }
    }
}

pub fn handle_plain(event_map: HashMap<String, String>) {
    let event_name = match event_map.get("Event-Name") {
        Some(name) => name,
        None => ""
    };

    let subclass = match event_map.get("Event-Subclass") {
        Some(subclass) => subclass,
        None => ""
    };

    if event_name == "CUSTOM" && subclass == "sofia::register_failure" {
        let from_ip = event_map.get("network-ip").unwrap();
        statis::insert(from_ip);
    }

    println!("{:?}", event_map);
}

pub fn handle_event(cmd_s: &Sender<Cmd>, event: Event) {
    match event {
        Event::Request(request, _content) => {
            handle_request(request);
        }
        Event::Reply(reply, _content) => {
            handle_reply(cmd_s, reply);
        }
        Event::EventPlain(_plain, content) => {
            handle_plain(content);
        }
    }
}

pub fn start_esl() {
    let (cmd_s, cmd_r) = bounded::<Cmd>(1);
    let (event_s, event_r) = bounded::<Event>(1);
    let ticker = tick(Duration::from_secs(1));

    let mut esl = Esl::new("127.0.0.1".to_string(),
        "8021".to_string(),
        "ClueCon".to_string(), cmd_r, event_s);

    iptables::init();

    thread::spawn(move || {
        esl
            .start()
            .expect("Error connect to FreeSwitch");
    });

    let std_r = spawn_stdin_channel();

    loop {
        select! {
            recv(std_r) -> _line => {
            },
            recv(event_r) -> event => {
                let event = event.unwrap();
                handle_event(&cmd_s, event);
            },
            recv(ticker) -> _ => {
                statis::remove_older_than(10);
                let ips = statis::identify_attacker_ips();
                block_ips(ips);
                statis::dump();
            }
        }
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (s, r) = unbounded();

    std::thread::spawn(move || {
        let stdin = io::stdin();
        let handle = stdin.lock();

        for line in handle.lines() {
            match line {
                Ok(content) => {
                    if s.send(content).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    r
}

pub fn block_ips(ips: Vec<String>) {
    for ip in ips {
        if let Ok(_) = firewall::get_by_ip(&ip) {
            continue;
        };

        let _ = firewall::deny(&ip);
        println!("Blocking IP: {}", ip);
        iptables::deny_ip(&ip);
    }
}

