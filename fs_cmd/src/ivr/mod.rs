use super::customtable::{Ctable};
use structopt::StructOpt;
use super::fs_lib::*;
use super::domain;

#[derive(StructOpt)]
#[structopt(about="Manage IVR")]
#[derive(Debug)]
pub enum IvrEntryCli {
    #[structopt(about="Add IVR entry")]
    Add {
        #[structopt(short, long, help="IVR id")]
        ivr_id: i32,
        #[structopt(long, help="Digits")]
        digits: String,
        #[structopt(long, help="Dest extension")]
        dest_exten: String,
    },
    #[structopt(about="Delete IVR entry")]
    Del {
    },
    #[structopt(about="List IVR entries")]
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

    Entry {
        #[structopt(subcommand, help="manager IVR options")]
        entry: IvrEntryCli,
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

pub fn exec_ivr_option_cmd(entry: IvrEntryCli) {
    let domain_id = domain::get_active().unwrap();
    match entry {
        IvrEntryCli::Add {ivr_id, digits, dest_exten} =>  {
            ivr::add_ivr_option(domain_id, ivr_id, digits, dest_exten).unwrap();
        },
        IvrEntryCli::Del {} => {},
        IvrEntryCli::Ls {} => {
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
        IvrCli::Entry { entry } => {
            exec_ivr_option_cmd(entry);
        }
    }
}
