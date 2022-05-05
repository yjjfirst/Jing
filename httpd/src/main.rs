mod xml_utils;

use std::io::BufWriter;
use std::collections::HashMap;
use tide::Request;
use tide::prelude::*;
use xml::writer::{EmitterConfig};

use self::xml_utils::*;
#[derive(Deserialize, Serialize, Debug)]
struct Configuration {
    hostname: String,
    section: String,
    tag_name: String,
    key_name: String,
    key_value: String
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/fsapi").post(fs_post);
    app.listen("0.0.0.0:9090").await?;
    Ok(())
}

async fn fs_post(mut req: Request<()>) -> tide::Result {
    let conf: Configuration  = req.body_form().await?;
    let mut buf = BufWriter::new(Vec::new());
    let mut w = EmitterConfig::new().perform_indent(true).create_writer(&mut buf);

    start_element(&mut w, "document", HashMap::from([("type", "freeswitch/xml")]));
    start_element(&mut w, "sectiont", HashMap::from([("name", "configuration")]));
    start_element(&mut w, "configuration", HashMap::from([
        ("name", "sofia.conf"),
        ("description","sofia Endpoint")
    ]));

    end_element(&mut w);
    end_element(&mut w);
    end_element(&mut w);

    let response = buf.into_inner().unwrap();
    let response: String = String::from_utf8(response).unwrap();

    Ok(response.into())
}
