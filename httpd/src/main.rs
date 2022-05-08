mod xml_utils;
mod configuration;

use tide::{Request, StatusCode};
use tide::prelude::*;

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

    if conf.section == "configuration" {
        configuration::serve()
    } else {
        return Err(tide::Error::from_str(StatusCode::NotFound, "Invalidated section"));
    }
}
