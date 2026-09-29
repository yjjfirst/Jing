pub fn deny_ip(ip: &str) {
    std::process::Command::new("iptables")
        .arg("-t")
        .arg("raw")
        .arg("-A")
        .arg("PREROUTING")
        .arg("-s")
        .arg(ip)
        .arg("-j")
        .arg("DROP")
        .output()
        .expect("Failed to execute command");
}

pub fn allow_ip(ip: &str) {
    std::process::Command::new("iptables")
        .arg("-t")
        .arg("raw")
        .arg("-D")
        .arg("PREROUTING")
        .arg("-s")
        .arg(ip)
        .arg("-j")
        .arg("DROP")
        .output()
        .expect("Failed to execute command");
}

pub fn clear() {
    println!("Clearing firewall...");

    std::process::Command::new("iptables")
        .arg("-t")
        .arg("raw")
        .arg("-F")
        .arg("PREROUTING")
        .output()
        .expect("Failed to execute command");
}

pub fn init() {
    let rules = super::list().unwrap();

    println!("Initializing firewall...");
    for rule in rules {
        if rule.action == "allow" {
            continue;
        }
        deny_ip(&rule.ip_address);
    }      
}