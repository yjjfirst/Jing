use std::cell::RefCell;
use std::collections::HashMap;
use chrono::{Local, DateTime, Duration};

thread_local! {
    static GLOBAL_MAP: RefCell<HashMap<String, Vec<DateTime<Local>>>> = RefCell::new(HashMap::new());
}

pub fn insert(ip_addr: &str) {
    GLOBAL_MAP.with(|map|{
        let now = Local::now();
        let mut borrowed = map.borrow_mut();
        borrowed.entry(ip_addr.to_string())
            .or_default()
            .push(now);
    })
}

pub fn remove_older_than(minutes: i64) {
    GLOBAL_MAP.with(|map|{
        let mut keys_to_remove: Vec<String> = Vec::new();
        let mut borrowed = map.borrow_mut();
        for (ip, date_vec) in borrowed.iter_mut() {
            date_vec.retain(|&d| {
                d > Local::now() - Duration::minutes(minutes)
            });
            if date_vec.len() == 0 {
                keys_to_remove.push(ip.to_string());
            }
        }

        for key in keys_to_remove {
            borrowed.remove(&key);
        }
    })
}

pub fn get_attacker_ips() -> Vec<String> {
    let mut attackers: Vec<String> = vec![];
    GLOBAL_MAP.with(|map|{
        let mut borrowed = map.borrow_mut();
        for (ip, date_vec) in borrowed.iter_mut() {
            if date_vec.len() > 5 {
                date_vec.clear();
            }

            attackers.push(ip.to_string());
        }
    });
    
    attackers
}

pub fn dump() {
    GLOBAL_MAP.with(|map|{
        let borrowed = map.borrow();
        if borrowed.len() == 0 {
            return;
        }
        println!("------------------------");
        for (ip, date_vec) in borrowed.iter() {
            println!("IP: {} Count: {}", ip, date_vec.len());
        }
    })
}