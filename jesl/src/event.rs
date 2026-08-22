use std::net::{TcpStream};
use std::io::{BufReader, BufRead, Read};
use std::sync::mpsc;
use std::collections::HashMap;
use urlencoding::decode_binary;

#[derive(Debug, Clone)]
pub enum Event {
    Request(Request, HashMap::<String, String>),
    Reply(Reply, HashMap::<String, String>),
    EventPlain(EventPlain,HashMap::<String, String>),
}

#[derive(Debug, Clone)]
pub enum Request {
    Auth,
}

#[derive(Debug, Clone)]
pub enum Reply {
    Command {status: String, text: String},
}

#[derive(Debug, Clone)]
pub enum EventPlain {
    Text
}


const COLON: &str = ":";
const SPACE: &str = " ";
const SLASH: &str = "/";

const REQUEST: &str = "request";
const REPLY: &str = "reply";
const EVENT_PLAIN: &str = "event-plain";

pub fn recv(stream: TcpStream, sender: mpsc::Sender<Event>) {    
    let mut reader = BufReader::new(stream);
    let mut lines: Vec<String> =Vec::new();

    loop {
        let mut buff = String::new();
        let len_result = reader.read_line(&mut buff);
        match len_result {
            Ok(line_len) => {
                lines.push(buff.trim().to_string());
                if line_len == 1 {
                    let headers = parse_headers(&lines);
                    println!("{:?}", headers);
                    
                    let content_len = headers
                        .get("Content-Length");
                    
                    let mut content = vec![0u8;0];

                    if let Some(len) = content_len {
                        let length = len.parse::<usize>().unwrap();
                        content.resize(length, 0u8);
                        reader.read_exact(&mut content).unwrap();
                    }
                    
                    let decoded_content = decode_binary(&content);
                    
                    if let Ok(event) = parse_event(headers, String::from_utf8_lossy(&decoded_content).to_string()) {
                        sender.send(event).unwrap();
                    }
                    
                    lines.clear();
                }

            }
            Err(e) =>  {
                println!("{}", e);
            }
        }
    }
}

fn parse_reply_text(reply: &str) -> (&str, &str){
    let values = reply.split_once(SPACE);
    match values {
        Some(values) => {
            (values.0, values.1)
        }
        
        None => {
            ("","")
        }            
    }

}

fn parse_content_type(content: &str) -> (&str, &str){
    let values: Vec<&str> = content
        .trim()
        .split(SLASH)
        .collect();
    
    let event_type = values[1];
    let event = values[0];

    (event_type, event)
}

fn parse_headers(lines: &Vec<String>) -> HashMap<String, String> {
    let mut headers: HashMap<String, String>  = HashMap::new(); 
    
    for l in lines {        
        let h = parse_header(l);
        if h.len() >= 2 {
            headers.insert(String::from(h[0].trim()),
                           String::from(h[1].trim()));
        }
    }

    headers
}

fn parse_header(line: &str) -> Vec<&str> {
    let header: Vec<&str> = line.split(COLON).collect();

    header
}


fn parse_event(headers: HashMap<String, String>, content: String) -> Result<Event, String> {
    let (event_type, event) = parse_content_type(headers.get("Content-Type").unwrap());
    let mut content_map: HashMap<String, String> = HashMap::new();

    if content.len() > 0 {
        println!("{}", content);
        let c = content.lines().map(String::from).collect();
        content_map = parse_headers(&c);        
    }

    return match  event_type {
        REQUEST => {
            match event {
                "auth" => {
                    Ok(Event::Request(Request::Auth, content_map))
                }
                _ => {
                    Err("parse event error".to_string())
                }
            }
        }
        REPLY => {
            match event {
                "command" => {
                    let (s, t) = parse_reply_text(headers.get("Reply-Text").unwrap());
                    Ok(Event::Reply(
                        Reply::Command {
                            status: s.to_string(),
                            text: t.to_string()
                        }, content_map))
                }
                _ => {
                    Err("parse event error".to_string())
                }
            }
        }
        EVENT_PLAIN => {
            match event {
                "text" => {
                    Ok(Event::EventPlain(EventPlain::Text, content_map))
                }
                _ => {
                    Err("parse event error".to_string())
                }

            }
        }
        _ => {
            Err("parse event error".to_string())
        }
    }    
}

