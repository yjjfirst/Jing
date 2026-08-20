use super::customtable::{Ctable};
use structopt::StructOpt;
use super::jlib::*;
use super::domain;

#[derive(StructOpt)]
#[derive(Debug)]

pub enum SoundFileCli {
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

pub fn print_soundfiles(sounds: Vec<sound_file::models::SoundFile>)
{
    let mut table = Ctable::new();

    table.set_titles(row!["id", "name", "domain id","description"]);
    for s in sounds {
        table.add_row(row![s.id,
                           s.name,
                           s.domain_id,
                           s.description.unwrap_or("".to_string())]);
    }

    table.print();
}

pub fn exec_soundfile_cmd(soundfile: SoundFileCli) {
    let domain_id = domain::get_active().unwrap();
    match soundfile {
        SoundFileCli::Add {name, path, desc} => {
            sound_file::add(domain_id, name, path, desc).unwrap();
        },
        SoundFileCli::Import {path} => {
            sound_file::import(domain_id, path).unwrap();
        },
        SoundFileCli::Del {id} =>{
            sound_file::del(id).unwrap();
        },
        SoundFileCli::Ls => {
            let soundfiles = sound_file::list().unwrap();
            print_soundfiles(soundfiles);
        }
    }
}
