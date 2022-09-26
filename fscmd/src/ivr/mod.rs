use super::customtable::{Ctable};
use structopt::StructOpt;
use super::fslib::*;

#[derive(StructOpt)]
#[structopt(about="Manage IVR")]
#[derive(Debug)]
pub enum IvrOptionCli {
    #[structopt(about="Add IVR option")]
    Add {
        #[structopt(short, long, help="IVR id")]
        ivr_id: i32,
        #[structopt(long, help="Digits")]
        digits: String,
        #[structopt(long, help="Dest extension")]
        dest_exten: String,
    },
    #[structopt(about="Delete IVR option")]
    Del {
    },
    #[structopt(about="List IVR options")]
    Ls {
    }
}

#[derive(StructOpt)]
#[structopt(about="Manage IVR")]
#[derive(Debug)]
pub enum IvrCli {
    #[structopt(about="Add IVR.")]
    Add {
        #[structopt(short, long, help="Name of the IVR")]
        name: String,
        #[structopt(short, long, help="Extension of the IVR")]
        exten: String,
        #[structopt(short, long, help="Domain of the IVR")]
        domain_id: i32,
        #[structopt(short="l", long, help="Long greet")]
        greet_long: Option<String>,
        #[structopt(short="s", long, help="Short greet")]
        greet_short: Option<String>,
        #[structopt(short, long, help="Invalid sound")]
        invalid_sound: Option<String>,
        #[structopt(short="x", long, help="Exit sound")]
        exit_sound: Option<String>

    },
    #[structopt(about="Delete IVR.")]
    Del {
        #[structopt(short, long, help="Database ID of the IVR")]
        id: i32,
    },
    #[structopt(about="List all IVRs.")]
    Ls {
    },

    Option {
        #[structopt(subcommand, help="manager IVR options")]
        option: IvrOptionCli,
    }
}

pub fn print_ivrs(ivrs: Vec<ivr::models::Ivr>) {
    let mut table = Ctable::new();

    table.set_titles(row!["id", "name", "exten", "domain_id", "greet_long", "greet_short", "invalid_sound", "exit_sound"]);
    for i in ivrs {
        table.add_row(
            row![
                i.id,
                i.name,
                i.exten,
                i.domain_id,
                i.greet_long.unwrap_or("".to_string()),
                i.greet_short.unwrap_or("".to_string()),
                i.invalid_sound.unwrap_or("".to_string()),
                i.exit_sound.unwrap_or("".to_string())
            ])
    }

    table.print();
}

pub fn exec_ivr_option_cmd(option: IvrOptionCli) {
    match option {
        IvrOptionCli::Add {ivr_id, digits, dest_exten} =>  {
            ivr::add_ivr_option(ivr_id, digits, dest_exten).unwrap();
        },
        IvrOptionCli::Del {} => {},
        IvrOptionCli::Ls {} => {
        }
    }

}

pub fn exec_ivr_cmd(ivr: IvrCli) {
    match ivr {
        IvrCli::Add {name, exten, domain_id, greet_long, greet_short, invalid_sound, exit_sound} => {
            use ivr::models::NewIvr;
            ivr::add(NewIvr {
                name: &name,
                exten: &exten,
                domain_id,
                greet_long: greet_long.as_deref(),
                greet_short: greet_short.as_deref(),
                invalid_sound: invalid_sound.as_deref(),
                exit_sound: exit_sound.as_deref()}).unwrap();
        },
        IvrCli::Del {id} => {
            ivr::del(id).unwrap();
        },
        IvrCli::Ls {} => {
            let ivrs = ivr::all().unwrap();
            print_ivrs(ivrs);
        },
        IvrCli::Option { option } => {
            exec_ivr_option_cmd(option);
        }
    }
}
