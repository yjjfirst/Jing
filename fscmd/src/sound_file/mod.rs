use super::customtable::{Ctable};
use structopt::StructOpt;
use super::fslib::*;

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
    Del {
        #[structopt(short, long)]
        id: i32,
    }
}

pub fn print_sounds(sounds: Vec<sound_file::models::SoundFile>)
{
    let mut table = Ctable::new();

    table.set_titles(row!["id", "name", "domain id","description"]);
    for s in sounds {
        table.add_row(row![s.id,
                           s.name,
                           s.domain_id,
                           s.desc.unwrap_or("".to_string())]);
    }

    table.print();
}

pub fn exec_sound_cmd(soundfile: SoundFileCli) {
    match soundfile {
        SoundFileCli::Add {name, path, desc} => {
            sound_file::add(name, path, desc).unwrap();
        },
        SoundFileCli::Del {id} =>{
            sound_file::del(id).unwrap();
        },
        SoundFileCli::Ls => {
            let sounds = sound_file::all().unwrap();
            print_sounds(sounds);
        }
    }
}
