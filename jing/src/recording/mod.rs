use super::customtable::{Ctable};
use structopt::StructOpt;
use jing_lib::*;
use super::domain;

#[derive(StructOpt)]
#[derive(Debug)]

pub enum RecordingCli {
    Ls,
    Add {
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        path: String,
        #[structopt(short, long)]
        desc: String,
    },
    Import {
        #[structopt(short, long)]
        path: String,
    },
    Del {
        #[structopt(short, long)]
        id: i32,
    }
}

pub fn print_recordings(recordings: Vec<recording::models::Recording>)
{
    let mut table = Ctable::new();

    table.set_titles(row!["id", "name", "domain id","description"]);
    for s in recordings {
        table.add_row(row![s.id,
                           s.name,
                           s.domain_id,
                           s.description.unwrap_or("".to_string())]);
    }

    table.print();
}

pub fn exec_recording_cmd(recording: RecordingCli) {
    let domain_id = domain::get_active().unwrap();
    match recording {
        RecordingCli::Add {name, path, desc} => {
            recording::add(domain_id, name, path, desc).unwrap();
        },
        RecordingCli::Import {path} => {
            recording::import(domain_id, path).unwrap();
        },
        RecordingCli::Del {id} =>{
            recording::del(id).unwrap();
        },
        RecordingCli::Ls => {
            let recordings = recording::list().unwrap();
            print_recordings(recordings);
        }
    }
}
