use std::env;

use super::customtable::{Ctable};
use structopt::StructOpt;
use super::fs_lib::*;

#[derive(StructOpt)]
#[derive(Debug)]
pub enum DomainCli {
    Ls,
    Add {
        #[structopt(short, long)]
        name: String,
    },
    Del {
        #[structopt(short, long)]
        id: i32,
    },
    Active {
        #[structopt(short, long)]
        id: Option<i32>,
    }
}

const ENV_VAR_NAME: &str = "FS_ACTIVE_DOMAIN";

fn print_domains(domains: Vec<domain::models::Domain>) {
    let mut table = Ctable::new();

    table.set_titles(row!["Id", "Domain Name"]);
    for d in domains {
        table.add_row(row![d.id, d.domain_name]);
    }

    table.print();

}

pub fn exec_domain_cmd(domain: DomainCli) {
    match domain {
        DomainCli::Add { name }=> {
            domain::add_domain(&name)
                .unwrap_or_else(|err| println!("{}", err));
        },
        DomainCli::Del { id } => {
            domain::del_domain(id)
                .unwrap_or_else(|err| println!("{}", err));
        },
        DomainCli::Active { id } => {
            active(id);
        },
        DomainCli::Ls => {
            ls();
        }
    }
}

fn active(id: Option<i32>) {
    if let Some(id) = id {
        env::set_var(ENV_VAR_NAME, id.to_string());
    } else {
        let active = env::var(ENV_VAR_NAME);
        match active {
            Ok(domain) => {
                println!("Active domain: {}", domain);
            },
            Err(_) => {
                no_active_domain();
            }
        }
    }
}

fn ls() {
    match domain::list_domains() {
        Ok(domains) => print_domains(domains),
        Err(err) => println!("{}", err),
    }
}

pub fn get_active() -> Result<i32, Box<dyn std::error::Error>> {
    let domain = env::var(ENV_VAR_NAME)?;
    let domain: i32 = domain.parse()?;

    Ok(domain)
}

pub fn no_active_domain() {
    println!("Available domains:");
    ls();
    println!("Please execute next command to setup active domain:");
    println!("export {}={{DOMAIN_ID}}", ENV_VAR_NAME);
}
