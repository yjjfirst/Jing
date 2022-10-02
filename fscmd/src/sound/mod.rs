use super::customtable::{Ctable};
use structopt::StructOpt;
use super::fslib::*;

#[derive(StructOpt)]
#[derive(Debug)]

pub enum SoundCli {
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

pub fn print_sounds(sounds: Vec<sound::models::Sound>)
{
    let mut table = Ctable::new();

    table.set_titles(row!["id", "name", "path", "domain id","description"]);
    for s in sounds {
        table.add_row(row![s.id,
                           s.name,
                           s.path,
                           s.domain_id,
                           s.desc.unwrap_or("".to_string())]);
    }

    table.print();
}

pub fn exec_sound_cmd(sound: SoundCli) {
    match sound {
        SoundCli::Add {name, path, desc} => {
            sound::add(name, path, desc).unwrap();
        },
        SoundCli::Del {id} =>{
            sound::del(id).unwrap();
        },
        SoundCli::Ls => {
            let sounds = sound::all().unwrap();
            print_sounds(sounds);
        }
    }
}
