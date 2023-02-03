use super::customtable::{Ctable};
use structopt::StructOpt;
use super::fslib::*;
use super::fslib::conference::conference_profile as profile;
use super::fslib::conference::conference_profile_param as param;
use super::domain;

#[derive(StructOpt)]
#[derive(Debug)]
pub enum ProfileCli {
    Add {
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        desc: String,
    },
    Del {
        #[structopt(short, long)]
        id: i32,
    },
    Update {
        #[structopt(short, long)]
        id: i32,
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        desc: String,
    },
    Ls,
    Param  {
        #[structopt(subcommand)]
        param: ParamCli,
    }
}
#[derive(StructOpt)]
#[derive(Debug)]
pub enum ParamCli {
    Add {
        #[structopt(short, long)]
        profile_id: i32,
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        value: String,
    },
    Del {
        #[structopt(short, long)]
        id: i32,
    },
    Update {
        #[structopt(short, long)]
        id: i32,
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        value: String,
    },
    Ls
}

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
    },
    Profile {
        #[structopt(subcommand)]
        profile: ProfileCli,
    }
}

pub fn exec_conference_cmd(conference: ConferenceCli) {
    let domain_id = domain::get_active().unwrap();
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
        },
        ConferenceCli::Profile { profile } => {
            exec_conference_profile_cmd(profile);
        }
    }
}

fn exec_conference_profile_cmd(profile: ProfileCli) {
    match profile {
        ProfileCli::Add {name, desc} => {
            profile::add(&name, &desc).unwrap();
        },
        ProfileCli::Del {id} => {
            profile::del(id).unwrap();
        },
        ProfileCli::Update {id, name, desc} => {
            profile::update(id, name, desc).unwrap();
        },
        ProfileCli::Ls => {
            let profiles = profile::profiles().unwrap();
            print_conference_profiles(profiles);
        },
        ProfileCli::Param {param} => {
            exec_conference_profile_param_cmd(param);
        }
    }
}

fn exec_conference_profile_param_cmd(param: ParamCli) {
    match param {
        ParamCli::Add {profile_id, name, value} => {
            param::ConferenceProfileParam::add(profile_id, &name, &value).unwrap();
        },
        ParamCli::Del {id} => {
            param::ConferenceProfileParam::del(id).unwrap();
        },
        ParamCli::Update {id, name, value} => {
            param::ConferenceProfileParam::update(id, &name, &value).unwrap();
        },
        ParamCli::Ls {..} => {
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

fn print_conference_profiles(profiles: Vec<profile::ConferenceProfile>) {
    let mut table = Ctable::new();
    table.set_titles(row!["id", "name","description"]);
    for c in profiles {
        let desc = c.description.unwrap_or("".to_string());
        table.add_row(row![
            c.id,
            c.name,
            desc
        ]);
    }

    table.print();
}
