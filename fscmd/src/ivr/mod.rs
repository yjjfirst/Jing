use super::customtable::{Ctable};
use structopt::StructOpt;
use super::fslib::*;
use super::domain;

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

pub fn print_ivrs(ivrs: Vec<ivr::Ivr>) {
    let mut table = Ctable::new();

    table.set_titles(row!["id", "name", "exten", "domain_id"]);
    for i in ivrs {
        table.add_row(
            row![
                i.id,
                i.name,
                i.exten,
                i.domain_id,
            ])
    }

    table.print();
}

pub fn exec_ivr_option_cmd(option: IvrOptionCli) {
    let domain_id = domain::get_active().unwrap();
    match option {
        IvrOptionCli::Add {ivr_id, digits, dest_exten} =>  {
            ivr::add_ivr_option(domain_id, ivr_id, digits, dest_exten).unwrap();
        },
        IvrOptionCli::Del {} => {},
        IvrOptionCli::Ls {} => {
        }
    }

}

pub fn exec_ivr_cmd(ivr: IvrCli) {
    let domain_id = domain::get_active().unwrap();
    match ivr {
        IvrCli::Add {name, exten} => {
            ivr::add(&name, &exten, domain_id).unwrap();
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
