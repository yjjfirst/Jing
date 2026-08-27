#[derive(Debug)]
pub enum Cmd {
    Auth {passwd: String},
    Event {format: String, event_type: String},
    Filter {event_name: String},
}

impl Cmd {
    pub fn to_string(&self) -> String {
        match self {
            Cmd::Auth { passwd } => {
                let r = String::new();
                let result = r + "Auth " + passwd;
                
                result
            }

            Cmd::Event { format, event_type } => {
                let r = String::new();
                let result = r + "event " + format + " " + event_type;

                result
            }

            Cmd::Filter { event_name } => {
                let r = String::new();
                let result = r + "filter " + "Event-Name " + event_name;

                result
            }
        }
    }
}

