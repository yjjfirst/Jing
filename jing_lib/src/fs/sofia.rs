use std::process::Command;
use regex::Regex;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayStatus {
    pub status: String,
    pub up_seconds: i32,
    pub in_total: i32,
    pub in_failed: i32,
    pub out_total: i32,
    pub out_failed: i32,
}
impl GatewayStatus {
    pub fn new() -> Self {
        GatewayStatus {
            status:"".to_string(),
            up_seconds: 0,
            in_total:0,
            in_failed:0,
            out_total:0,
            out_failed:0,
        }
    }
}

pub fn gateway(g_name: &str) -> GatewayStatus {
    let output = Command::new("fs_cli")
        .arg("-x")
        .arg(format!("sofia status gateway {}", g_name))
        .output()
        .expect("Failed to execute sofia gateway status command");

    let res = String::from_utf8(output.stdout)
        .expect("Failed to execute sofia gateway status");
    let res = res.trim().to_string();
    let status = get_header_value(&res, "Status",);

    let mut up_seconds = get_header_value(&res, "Uptime");
    up_seconds.pop();
    let up_seconds = up_seconds
            .parse::<i32>()
            .unwrap_or_default();

    let in_total = get_header_value(&res, "CallsIN")
            .parse::<i32>()
            .unwrap_or_default();

    let out_total = get_header_value(&res, "CallsOUT")
            .parse::<i32>()
            .unwrap_or_default();

    let in_failed = get_header_value(&res, "FailedCallsIN")
            .parse::<i32>()
            .unwrap_or_default();

    let out_failed = get_header_value(&res, "FailedCallsOUT")
            .parse::<i32>()
            .unwrap_or_default();

    GatewayStatus {
        status,
        up_seconds,
        in_total,
        in_failed,
        out_total,
        out_failed
    }
}

pub fn get_header_value(headers: &str, header_name: &str) -> String {
    let pattern = format!(r"(?m)^{}:?[ \t]*(.*)$", header_name);
    let re = Regex::new(&pattern).unwrap();
    let value = if let Some(caps) = re.captures(headers) {
        caps.get(1).map_or("",|m|m.as_str())
    } else {
        ""
    };

    value.to_string()
}