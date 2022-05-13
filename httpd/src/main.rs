mod xml_utils;
mod configuration;
mod directory;
mod dialplan;

use tide::{Request, StatusCode};
use tide::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct FsRequest {
    hostname: String,
    section: String,
    tag_name: String,
    key_name: String,
    key_value: String,
    #[serde(rename = "Caller-Destination-Number")]
    dest_number: Option<String>,
    #[serde(rename = "Caller-Context")]
    context: Option<String>,
    #[serde(rename = "variable_requested_domain_name")]
    dest_domain: Option<String>
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/fsapi").post(fs_post);
    app.listen("0.0.0.0:9090").await?;
    Ok(())
}

async fn fs_post(mut req: Request<()>) -> tide::Result {
    let fs_req: FsRequest  = req.body_form().await?;

    println!("{:?}", fs_req);
    if fs_req.section == "configuration" {
        configuration::serve()
    } else if fs_req.section == "directory" {
        directory::serve()
    } else if fs_req.section == "dialplan" {
        dialplan::serve(fs_req)
    } else {
        return Err(tide::Error::from_str(StatusCode::NotFound, "Invalidated section"));
    }
}
