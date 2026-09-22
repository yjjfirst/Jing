use std::process::Command;
use regex::Regex;
use std::collections::HashMap;

pub struct UserStatus {
    pub ip_addr: String,
    pub agent: String,
    pub ping: String
}

pub fn reg(domain: String) -> HashMap<String, UserStatus> {
    let output = Command::new("fs_cli")
        .arg("-x")
        .arg("sofia status profile internal reg")
        .output()
        .expect("Failed to execute sofia reg command");

    let res = String::from_utf8(output.stdout)
        .expect("Failed to execute sofia reg command");

    let res = res.trim().to_string();
    let re = Regex::new(r"\n[ \t]*\n").unwrap();
    let blocks: Vec<&str> = re.split(&res)
        .map(|s| s.trim())
        .filter(|s| {
            let pattern = format!(r"(?m)^User:[ \t]*\d*@{}$", domain);
            let re = Regex::new(&pattern).unwrap();

            if re.is_match(s) {
                true
            } else {
                false
            }

        })
        .collect();
    let mut user_map: HashMap<String, UserStatus> = HashMap::new();
    for b in blocks {
        let user = get_header_value(b, "Auth-User");
        let ip_addr = get_header_value(b, "IP");
        let agent = get_header_value(b, "Agent");
        let ping = get_header_value(b, "Ping-Status");

        user_map.insert(user, UserStatus {
            ip_addr, agent, ping
        });

    }

    user_map
}

pub fn get_header_value(headers: &str, header_name: &str) -> String {
    let pattern = format!(r"(?m)^{}:[ \t]*(.*)$", header_name);
    let re = Regex::new(&pattern).unwrap();
    let value = if let Some(caps) = re.captures(headers) {
        caps.get(1).map_or("",|m|m.as_str())
    } else {
        ""
    };

    value.to_string()
}
