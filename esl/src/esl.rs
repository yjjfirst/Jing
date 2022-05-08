use std::io::prelude::*;
use std::thread;
use std::net::{TcpStream};
use std::sync::mpsc;
use std::collections::HashMap;

use super::event::*;
use super::cmd::*;
use fslib::cdr;

#[derive(Debug)]
pub struct Bleg {
    caller_id: String,
    dest: String,
    a_uuid: String,
}

pub struct Esl {
    password: String,
    ipaddr: String,
    port: String,
    tcp_send_stream: Option<TcpStream>,
    last_cmd: Option<Cmd>,
    cdr_blegs: Vec<Bleg>,
}

impl Esl {
    pub fn new(ipaddr: String, port:String, password: String) -> Esl {
        Esl {
            password,
            ipaddr,
            port,
            tcp_send_stream: None,
            last_cmd: None,
            cdr_blegs: Vec::new(),
        }
    }

    pub fn send_line(&mut self, line: &[u8]) {
        let stream = self.tcp_send_stream.as_ref();
        stream
            .unwrap()
            .write(line)
            .unwrap();
        self.send_ending();
    }

    pub fn send_ending(&mut self) {
        let ending = b"\r\n";
        let stream = self.tcp_send_stream.as_ref();
        stream
            .unwrap()
            .write(ending)
            .unwrap();
    }

    pub fn send_auth(&mut self) {
        let cmd = Cmd::Auth {passwd: String::from(&self.password)};
        self.send(cmd);
    }

    pub fn enable_event(&mut self) {
        let cmd = Cmd::Event { format: String::from("plain"), event_type: String::from("ALL")};
        self.send(cmd);
    }

    pub fn enable_cdr(&mut self) {
        let cmd = Cmd::Filter {event_name: String::from("CHANNEL_HANGUP_COMPLETE")};
        self.send(cmd);
    }

    pub fn send(&mut self, cmd: Cmd) {
        match self.last_cmd {
            Some(_) => println!("Command executing, please try a little later."),
            None => {
                let msg = cmd.to_cmd_string();
                self.send_line(msg.as_bytes());
                self.send_ending();

                self.last_cmd = Some(cmd);
            }
        }

    }

    pub fn start(&mut self) -> Result<mpsc::Receiver<Event>, std::io::Error> {
        let (mpsc_sender, mpsc_receiver) = mpsc::channel::<Event>();
        let url = self.ipaddr.to_string() + ":" + &self.port;

        match TcpStream::connect(url) {
            Ok(tcp_stream) => {
                let  stream_clone = tcp_stream.try_clone();

                thread::spawn( move || {
                    recv(tcp_stream, mpsc_sender);
                });

                self.tcp_send_stream = Some(stream_clone.unwrap());
                Ok(mpsc_receiver)
            },
            Err(e) => {
                return Err(e);
            }
        }
    }

    fn handle_request(&mut self, request: Request) {
        match request {
            Request::Auth => {
                println!("Sending Auth...");
                self.send_auth();
            }
        }

    }

    fn handle_reply(&mut self, reply: Reply) {
        match reply {
            Reply::Command { status, text} => {
                if status == "+OK" {
                    if let Some(Cmd::Auth{..}) = self.last_cmd {
                        println!("Auth: {} {}", status, text);
                        self.last_cmd = None;
                        self.enable_event();
                    } else if let Some(Cmd::Event{..}) = self.last_cmd {
                        println!("Event {} {}", status, text);
                        self.last_cmd = None;
                        self.enable_cdr();
                    } else if let Some(Cmd::Filter{..}) = self.last_cmd {
                        println!("Filter {} {}", status, text);
                        self.last_cmd = None;
                    }

                } else {
                    println!("{} {}", status, text);
                }
            }
        }
    }

    pub fn add_leftover_bleg(&mut self, uuid: &str) {
        let mut i = 0;

        while i < self.cdr_blegs.len() {
            if self.cdr_blegs[i].a_uuid == uuid.to_string() {
                let b = self.cdr_blegs.remove(i);
                cdr::add_bleg(&b.caller_id,
                                  &b.dest,
                                  &b.a_uuid).unwrap();
            } else {
                i = i + 1;
            }
        }
    }

    pub fn handle_cdr_aleg(&mut self, content: HashMap<String, String>) {
        let caller_id =
            match content.get("Caller-Caller-ID-Number") {
                Some(cid) => cid,
                None => return
            };

        let dest =
            match content.get("Caller-Destination-Number") {
                Some(d) => d,
                None => return
            };

        let time = content
            .get("Caller-Channel-Created-Time")
            .unwrap();

        let (start_time, _) = time.split_at(10);
        let duration = content.get("variable_duration")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let uuid = content.get("Unique-ID").unwrap();
        cdr::add_cdr(caller_id, dest, start_time, duration, uuid).unwrap();

        self.add_leftover_bleg(&uuid);
    }

    pub fn handle_cdr_bleg(&mut self, content: HashMap<String, String>) {
        let uuid =
            match content.get("variable_originating_leg_uuid") {
                Some(uuid) => uuid,
                None => return
            };

        let caller_id =
            match content.get("Caller-Caller-ID-Number") {
                Some(cid) => cid,
                None => return
            };

        let dest =
            match content.get("Caller-Destination-Number") {
                Some(d) => d,
                None => return
            };

        match cdr::add_bleg(caller_id, dest, uuid) {
            Ok(_) => return,
            Err(_) => {
                let bleg =  Bleg {
                    caller_id: caller_id.to_string(),
                    dest: dest.to_string(),
                    a_uuid: uuid.to_string(),
                };

                self.cdr_blegs.push(bleg);
            }
        }
    }

    pub fn handle_plain(&mut self, content: HashMap<String, String>) {
        match content.get("variable_originating_leg_uuid") {
            Some(_) => self.handle_cdr_bleg(content),
            None =>  self.handle_cdr_aleg(content)
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Request (request, _content) => {
                self.handle_request(request);
            }
            Event::Reply (reply, _content) => {
                self.handle_reply(reply);
            }
            Event::EventPlain(_plain, content) => {
                self.handle_plain(content);
            }
        }
    }
}
