use std::process::Command;
use regex::Regex;

pub fn reload_mod(name: &str) {
    Command::new("fs_cli")
        .arg("-x")
        .arg(format!("reload {}", name))
        .spawn()
        .expect("Failed to reload module");
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

    let result = String::from_utf8(output.stdout).expect("Cannot connect to freeswitch");

    return result.trim().to_string();
}

pub fn is_var(name: &str) -> bool {
    let re = Regex::new(r"^\$\$\{.+\}$").unwrap();

    re.is_match(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_var() {
        assert_eq!(is_var("$${domain}"), true);
    }

    #[test]
    fn test_not_var() {
        assert_eq!(is_var("domain"), false);
    }

    #[test]
    fn test_not_var_2() {
        assert_eq!(is_var("$$domain"), false);
    }

}
