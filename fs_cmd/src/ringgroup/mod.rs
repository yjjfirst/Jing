use super::customtable::{Ctable};
use structopt::StructOpt;
use super::fs_lib::*;
use super::domain;

#[derive(StructOpt)]
#[structopt(about="Manage the ringing group member")]
#[derive(Debug)]
pub enum RgMemberCli {
    #[structopt(about="Add user to ringing group.")]
    Add {
        #[structopt(short, long, help="Ringing Group ID to add user to.")]
        group: i32,
        #[structopt(short, long, help="User ID to be added to Ringing Group.")]
        user: i32
    },
    #[structopt(about="Delete user from ringing group.")]
    Del {
        #[structopt(short, long, help="Ringing group ID")]
        group: i32,
        #[structopt(short, long, help="User ID")]
        user: i32
    },
    #[structopt(about="List all users in ringing group.")]
    Ls {
        #[structopt(short, long, help="Ringing Group ID")]
        group: i32,
    }
}

#[derive(StructOpt)]
#[derive(Debug)]
pub enum RgCli {
    Add {
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        group_id: String,
        #[structopt(short, long)]
        ring_time: Option<i32>,
        #[structopt(short, long)]
        strategy: Option<String>,
    },
    Del {
        #[structopt(short, long)]
        id: i32,
    },
    Ls,
    Member {
        #[structopt(subcommand)]
        member: RgMemberCli,
    }
}

pub fn print_ringing_groups(groups: Vec<ringgroup::models::Ringgroup>)  {
    let mut table = Ctable::new();

    table.set_titles(row!["id", "name", "group_id", "domain_id", "ring_time", "ring_strategy"]);
    for g in groups {
        table.add_row(
            row![
                g.id,
                g.name,
                g.group_id,
                g.domain_id,
                g.ring_time,
                g.ring_strategy
            ])
    }

    table.print();
}

pub fn exec_rg_member_cmd(member: RgMemberCli) {
    match member {
        RgMemberCli::Add {group, user}=> {
            ringgroup::add_ringgroup_member(group, user)
                .unwrap();
        },
        RgMemberCli::Del {group, user}=> {
            ringgroup::del_ringgroup_member(group, user)
                .unwrap();
        },
        RgMemberCli::Ls {group}=> {
            let members = ringgroup::members(group)
                .unwrap();
            let mut table = Ctable::new();

            table.set_titles(row!["User"]);
            for m in members {
                table.add_row(
                    row![
                        m,
                    ])
            }

            table.print();
        }
    }
}

pub fn exec_rg_cmd(rg: RgCli) {
    let domain_id = domain::get_active().unwrap();
    match rg {
        RgCli::Add {name, group_id, ring_time, strategy} => {
            ringgroup::add_ringgroup(domain_id, name, group_id, Some("".to_string()), ring_time, strategy)
                .unwrap();
        },
        RgCli::Del { id } => {
            ringgroup::del_ringgroup(id)
                .unwrap();
        },
        RgCli::Ls => {
            let groups = ringgroup::all_ringgroup()
                .unwrap();
            print_ringing_groups(groups);
        },
        RgCli::Member {member} => {
            exec_rg_member_cmd(member);
        },
    }
}
