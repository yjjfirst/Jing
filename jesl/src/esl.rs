use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::net::{TcpStream};
use crossbeam_channel::{Select, Receiver, Sender};

use super::event::*;
use super::cmd::*;

pub struct Esl {
    pub password: String,
    pub ipaddr: String,
    pub port: String,
    pub tcp_send_stream: Option<TcpStream>,
    pub cmd_r: Receiver<Cmd>,
    pub waiting_reply: bool,
}

pub fn enable_event(cmd_s: &Sender<Cmd>) {
    let cmd = Cmd::Event { format: String::from("plain"), event_type: String::from("ALL")};
    cmd_s.send(cmd).unwrap();
}

pub fn enable_cdr(cmd_s: &Sender<Cmd>) {
    let cmd = Cmd::Filter {event_name: String::from("CHANNEL_HANGUP_COMPLETE")};
    cmd_s.send(cmd).unwrap();
}

impl Esl {
    pub fn new(ipaddr: String, port:String, password: String, cmd_r: Receiver<Cmd>) -> Esl {
        Esl {
            password,
            ipaddr,
            port,
            tcp_send_stream: None,
            cmd_r,
            waiting_reply: false,
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

    pub fn send(&mut self, cmd: Cmd) {
        let msg = cmd.to_string();
        self.send_line(msg.as_bytes());
        self.send_ending();
    }

    pub fn start(&mut self, handler: fn(&mut Esl, Event)) -> Result<crossbeam_channel::Receiver<Event>, std::io::Error> {
        let (event_sender, event_receiver) = crossbeam_channel::bounded(1);
        let url = self.ipaddr.to_string() + ":" + &self.port;

        match TcpStream::connect(url) {
            Ok(tcp_stream) => {
                let  stream_clone = tcp_stream.try_clone();

                thread::spawn( move || {
                    recv(tcp_stream, event_sender);
                });

                self.tcp_send_stream = Some(stream_clone.unwrap());
                loop {
                    let mut sel = Select::new();
                    let event_id = sel.recv(&event_receiver);
                    let cmd_id = sel.recv(&self.cmd_r);
                    let ready_id = sel.ready();

                    if ready_id == event_id {
                        if let Ok(event) = event_receiver.try_recv() {
                            match event {
                                Event::Request(request,_) => {
                                    match request {
                                        Request::Auth => {
                                            println!("Sending Auth...");
                                            self.send_auth();
                                        }
                                    }
                                },
                                Event::Reply(_,_) => {
                                    self.waiting_reply = false;
                                    handler(self, event);
                                },
                                Event::EventPlain(_,_) => {
                                    handler(self, event);
                                }
                            }
                        }
                    } else if ready_id == cmd_id {
                        if self.waiting_reply == false {
                            if let Ok(cmd) = self.cmd_r.try_recv() {
                                self.waiting_reply = true;
                                self.send(cmd);                        
                            }
                        } else {
                            thread::sleep(Duration::from_micros(100));
                        }
                    }
                }
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
}
