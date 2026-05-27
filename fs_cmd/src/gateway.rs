use std::collections::HashMap;

use structopt::StructOpt;
use crate::fs_lib::gateway;
use crate::customtable::{Ctable};

#[derive(StructOpt)]
#[derive(Debug)]
pub enum GatewayCli {
    Ls,
    Add {
        #[structopt(long)]
        profile: i32,
        #[structopt(short, long)]
        name: String,
        #[structopt(long)]
        proxy: String,
        #[structopt(short, long)]
        register: String,
        #[structopt(short, long)]
        username: String,
        #[structopt(long)]
        password: String
    },
    Del {
        #[structopt(short, long)]
        id: i32
    }

}

pub fn exec_gateway_cmd(gateway: GatewayCli) {
    match gateway {
        GatewayCli::Add {profile, name, proxy, register, username, password} => {
            gateway::add(
                profile,
                name,
                [
                    ("proxy".to_string(), proxy),
                    ("register".to_string(), register),
                    ("username".to_string(), username),
                    ("password".to_string(), password)
                ]
                    .into_iter()
                    .collect::<HashMap<String, String>>()
            ).unwrap_or_else(|err| {
                println!("{}",err);
                0
            });

        },

        GatewayCli::Del {id} => {
            gateway::del(id)
                .unwrap_or_else(|err| println!("{}",err));
        },

        GatewayCli::Ls => {
            match gateway::list() {
                Ok(gateways) => print_gateways(gateways),
                Err(err) => println!("{}", err),
            }
        }
    }
}

fn print_gateways(gateways: Vec<gateway::models::Gateway>)  {

    let mut table = Ctable::new();

    table.set_titles(row!["Id", "Profile_id", "name"]);

    for g in gateways
    {
        table.add_row(row![g.id, g.profile_id, g.gateway_name]);
    }

    table.print();

}
