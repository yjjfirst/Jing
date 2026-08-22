use std::io::prelude::*;
use std::thread;
use std::net::{TcpStream};
use std::sync::mpsc;

use super::event::*;
use super::cmd::*;

pub struct Esl {
    pub password: String,
    pub ipaddr: String,
    pub port: String,
    pub tcp_send_stream: Option<TcpStream>,
    pub last_cmd: Option<Cmd>,
}

impl Esl {
    pub fn new(ipaddr: String, port:String, password: String) -> Esl {
        Esl {
            password,
            ipaddr,
            port,
            tcp_send_stream: None,
            last_cmd: None,
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
                let msg = cmd.to_string();
                self.send_line(msg.as_bytes());
                self.send_ending();

                self.last_cmd = Some(cmd);
            }
        }
    }

    pub fn start(&mut self, handler: fn(&mut Esl, Event)) -> Result<mpsc::Receiver<Event>, std::io::Error> {
        let (mpsc_sender, mpsc_receiver) = mpsc::channel::<Event>();
        let url = self.ipaddr.to_string() + ":" + &self.port;

        match TcpStream::connect(url) {
            Ok(tcp_stream) => {
                let  stream_clone = tcp_stream.try_clone();

                thread::spawn( move || {
                    recv(tcp_stream, mpsc_sender);
                });

                self.tcp_send_stream = Some(stream_clone.unwrap());
                while let Ok(event) = mpsc_receiver.recv() {
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
                            self.last_cmd = None;
                            handler(self, event);
                        },
                        Event::EventPlain(_,_) => {
                            handler(self, event);
                        }
                    }
                }
                Ok(mpsc_receiver)
            },

            Err(e) => {
                return Err(e);
            }
        }
    }
}
