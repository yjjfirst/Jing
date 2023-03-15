use super::customtable::{Ctable};
use fs_lib::printable::{Printable};
use structopt::StructOpt;
use super::fs_lib::*;
use super::fs_lib::conference::conference_profile as profile;
use super::fs_lib::conference::conference_profile_param as param;
use super::fs_lib::conference::conference_control as control;
use super::fs_lib::conference::conference_control_detail as detail;
use super::domain;
use crate::print_table;

#[derive(StructOpt)]
#[derive(Debug)]
pub enum ControlCli {
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
    Detail  {
        #[structopt(subcommand)]
        detail: DetailCli,
    }
}

#[derive(StructOpt)]
#[derive(Debug)]
pub enum DetailCli {
    Add {
        #[structopt(short, long)]
        control_id: i32,
        #[structopt(short, long)]
        action: String,
        #[structopt(short, long)]
        digits: String,
    },
    Del {
        #[structopt(short, long)]
        id: i32,
    },
    Update {
        #[structopt(short, long)]
        id: i32,
        #[structopt(short, long)]
        action: String,
        #[structopt(short, long)]
        digits: String,
    },
    Ls {
        #[structopt(short, long)]
        control_id: i32,
    }
}

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
    Ls {
        #[structopt(short, long)]
        profile_id: i32,
    }
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
        description: String
    },
    Del {
        #[structopt(short, long)]
        id: i32
    },
    Profile {
        #[structopt(subcommand)]
        profile: ProfileCli,
    },
    Control {
        #[structopt(subcommand)]
        control: ControlCli,
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
        },
        ConferenceCli::Control { control } => {
            exec_conference_control_cmd(control);
        }
    }
}

fn exec_conference_control_cmd(control: ControlCli) {
    match control {
        ControlCli::Add {name, desc} => {
            control::add(&name, &desc).unwrap();
        },
        ControlCli::Del {id } => {
            control::del(id).unwrap();
        },
        ControlCli::Update {id, name, desc} => {
            control::update(id, &name, &desc).unwrap();
        },
        ControlCli::Ls => {
            let groups = control::groups().unwrap();
            print_conference_controls(groups);
        },
        ControlCli::Detail {detail} => {
            exec_conference_control_detail_cmd(detail);
        }
    }
}

fn exec_conference_control_detail_cmd(detail: DetailCli) {
    match detail {
        DetailCli::Add {control_id, action, digits} => {
            detail::ConferenceControlDetail::add(control_id, &action, &digits).unwrap();
        },
        DetailCli::Del {id} => {
            detail::ConferenceControlDetail::del(id).unwrap();
        },
        DetailCli::Update {id, action, digits} => {
            detail::ConferenceControlDetail::update(id, &action, &digits).unwrap();
        },
        DetailCli::Ls {control_id} => {
            let details = detail::ConferenceControlDetail::belong_to(control_id).unwrap();
            print_table!(details);
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
        ParamCli::Ls {profile_id} => {
            let params = param::ConferenceProfileParam::belong_to(profile_id).unwrap();
            print_table!(params);
        }
    }
}

fn print_conferences(conferences: Vec<conference::Conference>) {
    print_table!(conferences);
}

fn print_conference_profiles(profiles: Vec<profile::ConferenceProfile>) {
    print_table!(profiles);
}

fn print_conference_controls(controls: Vec<control::ConferenceControl>) {
    print_table!(controls);
}
