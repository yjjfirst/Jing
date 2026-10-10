use super::customtable::{Ctable};
use structopt::StructOpt;
use jing_lib::*;
use super::domain;


#[derive(StructOpt)]
#[derive(Debug)]
pub enum AnnouncementCli {
    Ls,
    Add
    {
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        exten: String,
        #[structopt(short, long)]
        recording_id: i32
    },
    Del {
        #[structopt(short, long)]
        id: i32,
    }
}

pub fn print_announcements(announcements: Vec<announcement::models::Announcement>) {
    let mut table = Ctable::new();
    table.set_titles(row!["id", "name", "exten", "doamin_id", "recording_id"]);
    for s in announcements {
        table.add_row(row![
            s.id,
            s.name,
            s.exten,
            s.domain_id,
            s.recording_id
        ]);
    }

    table.print();
}

pub fn exec_announcement_cmd(announcement: AnnouncementCli) {
    let domain_id = domain::get_active().unwrap();
    match announcement {
        AnnouncementCli::Add {name, exten, recording_id} => {
            announcement::add(domain_id, recording_id, name, exten).unwrap();
        },
        AnnouncementCli::Del {id} => {
            announcement::del(id).unwrap();
        },
        AnnouncementCli::Ls => {
            let announcements = announcement::list().unwrap();
            print_announcements(announcements);
        }
    }
}
