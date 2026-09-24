pub mod sofia;

use serde::{Serialize, Deserialize};
use std::process::Command;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub direction: String,
    pub cid_name: String,
    pub cid_num: String,
    pub callee_num: String
}

pub fn reload_mod(name: &str) {
    Command::new("fs_cli")
        .arg("-x")
        .arg(format!("reload {}", name))
        .spawn()
        .expect(&format!("Failed to reload module: {}", name));
}

pub fn reload_acl() {
    Command::new("fs_cli")
        .arg("-x")
        .arg("reloadacl")
        .spawn()
        .expect("Failed to reload ACL");
}

pub fn eval(name: &str) -> String {
    let output = Command::new("fs_cli")
        .arg("-x")
        .arg(format!("eval {}", name))
        .output()
        .expect("Failed to evaluate variable");

    let result = String::from_utf8(output.stdout).expect("Failed to evaluate variable");

    return result.trim().to_string();
}

pub fn is_var(name: &str) -> bool {
    let re = Regex::new(r"^\$\$\{.+\}$").unwrap();

    re.is_match(name)
}

pub fn show_channels() -> Vec<Channel> {
    let output = Command::new("fs_cli")
        .arg("-x")
        .arg("show channels")
        .output()
        .expect("Show channels failed");

    let res = String::from_utf8(output.stdout)
        .expect("Show channels failed");

    let mut lines: Vec<&str> = res.lines().collect();
    if !lines[0].starts_with("uuid") {
        return vec![];
    }

    let mut channels: Vec<Channel> = vec![];
    lines.remove(0);
    lines.pop();
    lines.pop();
    lines.pop();

    for line in lines {
        let items: Vec<&str> = line.split(',').collect();
        let channel = Channel {
            direction: items[1].to_string(),
            cid_name: items[6].to_string(),
            cid_num: items[7].to_string(),
            callee_num: items[27].to_string()
        };

        channels.push(channel);
    }

    channels
}

