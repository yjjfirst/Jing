#[derive(Debug)]
pub enum Cmd {
    Auth {
        passwd: String
    },
    Event {
        format: String, 
        name: String,
        subclass: Option<String>
    },
    Filter {
        event_name: String
    },
}

impl Cmd {
    pub fn to_string(&self) -> String {
        match self {
            Cmd::Auth { passwd } => {
                let r = String::new();
                let result = r + "Auth " + passwd;
                
                result
            }

            Cmd::Event { format, name, subclass } => {
                let r = String::new();
                let mut cmd_string = r + "event " + format + " " + name;

                if let Some(sub) = subclass {
                    cmd_string = cmd_string + " " + sub;
                }

                cmd_string
            }

            Cmd::Filter { event_name } => {
                let r = String::new();
                let result = r + "filter " + "Event-Name " + event_name;

                result
            }
        }
    }
}

