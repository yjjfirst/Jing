use super::customtable::{Ctable};
use structopt::StructOpt;
use super::fslib::*;

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
            domain::add_domain(&name, false)
                .unwrap_or_else(|err| println!("{}", err));
        },

        DomainCli::Del { id } => {
            domain::del_domain(id)
                .unwrap_or_else(|err| println!("{}", err));
        },

        DomainCli::Active { id } => {
            if let Some(id) = id {
                domain::set_active(id)
                    .unwrap_or_else(|err| println!("{}", err));
            } else {
                println!("Active domain: {}", domain::get_active().unwrap().id);
            }
        },

        DomainCli::Ls => {
            match domain::list_domains() {
                Ok(domains) => print_domains(domains),
                Err(err) => println!("{}", err),
            }
        }
    }
}
