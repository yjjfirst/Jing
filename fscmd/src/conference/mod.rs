use super::customtable::{Ctable};
use structopt::StructOpt;
use super::fslib::*;
use super::domain;

#[derive(StructOpt)]
#[derive(Debug)]
pub enum ConferenceCli {
    Ls,
    Add {
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        exten: String,
        #[structopt(short, long)]
        conference_profile_id: i32,
        #[structopt(short, long)]
        description: Option<String>
    },
    Del {
        #[structopt(short, long)]
        id: i32
    }
}

pub fn exec_conference_cmd(conference: ConferenceCli) {
    let domain_id = domain::get_active()
        .expect("Please set active domain");
    match conference {
        ConferenceCli::Add {name, exten, conference_profile_id, description} => {
            conference::add(domain_id,
                            conference_profile_id,
                            exten,
                            name,
                            description).unwrap();
        },
        ConferenceCli::Del {id} => {
            conference::del(id).unwrap();
        },
        ConferenceCli::Ls => {
            let conferences = conference::all().unwrap();
            print_conferences(conferences);
        }
    }
}

fn print_conferences(conferences: Vec<conference::Conference>) {
    let mut table = Ctable::new();
    table.set_titles(row!["id", "name","exten", "domain_id", "conference_profile_id"]);
    for c in conferences {
        table.add_row(row![
            c.id,
            c.name,
            c.exten,
            c.domain_id,
            c.conference_profile_id
        ]);
    }

    table.print();
}
