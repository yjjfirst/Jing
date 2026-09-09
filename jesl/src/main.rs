mod statis;

use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::io;
use std::io::{BufRead};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crossbeam_channel::{bounded, unbounded, Sender, Receiver, select, tick};

use jeslib::esl::{ Esl, event, filter};
use jeslib::cmd::{ Cmd };
use jeslib::event::{ Event,Request,Reply };
use jlib::firewall;

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

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let (cmd_s, cmd_r) = bounded::<Cmd>(1);
    let (event_s, event_r) = bounded::<Event>(1);
    let ticker = tick(Duration::from_secs(1));

    let mut esl = Esl::new("127.0.0.1".to_string(),
        "8021".to_string(),
        "ClueCon".to_string(), cmd_r, event_s);

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    init_firewall();

    thread::spawn(move || {
        esl
            .start()
            .expect("Error connect to FreeSwitch");
    });
    
    let std_r = spawn_stdin_channel();
    
    while running.load(Ordering::SeqCst) {
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

    println!("Shutting down...");
    clear_firewall();
    println!("Shutdown complete.");

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
        if let Ok(true) = firewall::exists(&ip) {
            continue;
        }

        if let Ok(_) =firewall::deny(&ip) {
            println!("Blocked IP: {}", ip);
        }

        std::process::Command::new("ufw")
            .arg("insert")
            .arg("1")
            .arg("deny")
            .arg("from")
            .arg(ip)
            .output()
            .expect("Failed to execute command");
    }
}

pub fn clear_firewall() {
    let rules = firewall::list().unwrap();

    println!("Clearing firewall...");
    
    for rule in rules {
        if rule.action == "allow" {
            continue;
        }
        std::process::Command::new("ufw")
            .arg("delete")
            .arg("deny")
            .arg("from")
            .arg(rule.ip_address)
            .output()
            .expect("Failed to execute command");
    }
}

pub fn init_firewall() {
    let rules = firewall::list().unwrap();

    println!("Initializing firewall...");
    for rule in rules {
        if rule.action == "allow" {
            continue;
        }
        std::process::Command::new("ufw")
            .arg("insert")
            .arg("1")
            .arg("deny")
            .arg("from")
            .arg(rule.ip_address)
            .output()
            .expect("Failed to execute command");  
    }      
}