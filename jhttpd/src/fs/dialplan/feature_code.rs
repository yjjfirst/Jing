use std::io::Write;
use xml::writer::{EventWriter};
use crate::fs::xml_utils::action;
use jlib::feature_code::FeatureCode;

pub struct Feature {
    pub name: String,
    pub serve: fn(caller_id: Option<String>) -> Vec<(String, String)>
}


fn voicemail(_caller_id: Option<String>) -> Vec<(String, String)> {
    vec![
        ("answer".to_string(), "".to_string()),
        ("sleep".to_string(), "1000".to_string()),
        ("voicemail".to_string(), "check default ${domain_name}".to_string())
    ]
}

fn user_voicemail(caller_id: Option<String>) -> Vec<(String, String)> {
    let data = format!("check default ${{domain_name}} {:}", caller_id.unwrap());
    vec![
        ("answer".to_string(), "".to_string()),
        ("sleep".to_string(), "1000".to_string()),
        ("voicemail".to_string(), data)
    ]
}

pub fn serve<W: Write>(w: &mut EventWriter<W>, digits: String, caller_id: Option<String>) {
    let feature_codes: Vec<Feature> = vec![
        Feature {
            name: "voicemail".to_string(),
            serve: voicemail
        },
        Feature {
            name: "user_voicemail".to_string(),
            serve: user_voicemail
        }

    ];

    let feature = FeatureCode::get_by(digits).unwrap();
    let mut actions :Vec<(String, String)> = Vec::new();
    for f in feature_codes {
        if f.name == feature.action {
            actions = (f.serve)(caller_id);
            break;
        }
    }

    for a in actions {
        action(w, &a.0, &a.1);
    }
}
