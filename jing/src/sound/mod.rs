use super::customtable::{Ctable};
use structopt::StructOpt;
use super::jlib::*;
use super::domain;


#[derive(StructOpt)]
#[derive(Debug)]
pub enum SoundCli {
    Ls,
    Add
    {
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        exten: String,
        #[structopt(short, long)]
        sound_file_id: i32
    },
    Del {
        #[structopt(short, long)]
        id: i32,
    }
}

pub fn print_sounds(sounds: Vec<sound::models::Sound>) {
    let mut table = Ctable::new();
    table.set_titles(row!["id", "name", "exten", "doamin_id", "sound_file_id"]);
    for s in sounds {
        table.add_row(row![
            s.id,
            s.name,
            s.exten,
            s.domain_id,
            s.sound_file_id
        ]);
    }

    table.print();
}

pub fn exec_sound_cmd(sound: SoundCli) {
    let domain_id = domain::get_active().unwrap();
    match sound {
        SoundCli::Add {name, exten, sound_file_id} => {
            sound::add(domain_id, sound_file_id, name, exten).unwrap();
        },
        SoundCli::Del {id} => {
            sound::del(id).unwrap();
        },
        SoundCli::Ls => {
            let sounds = sound::list().unwrap();
            print_sounds(sounds);
        }
    }
}
