use super::customtable::{Ctable};
use structopt::StructOpt;
use super::fslib::*;


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

}

pub fn exec_sound_cmd(sound: SoundCli) {
    let domain = domain::get_active().unwrap();
    match sound {
        SoundCli::Add {name, exten, sound_file_id} => {
            sound::add(domain.id, sound_file_id, name, exten).unwrap();
        },
        SoundCli::Del {id} => {
            sound::del(id).unwrap();
        },
        SoundCli::Ls => {
            let sounds = sound::all().unwrap();
            print_sounds(sounds);
        }
    }
}
