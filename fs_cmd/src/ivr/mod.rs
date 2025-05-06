use super::customtable::{Ctable};
use structopt::StructOpt;
use super::fs_lib::*;
use super::domain;

#[derive(StructOpt)]
#[structopt(about="Manage IVR entry")]
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
        #[structopt(long, help="Id of the entry to be deleted")]
        id: i32,
    },
    #[structopt(about="List IVR entries")]
    Ls {
        #[structopt(short, long, help="IVR id")]
        ivr_id: i32,
    }
}
#[derive(StructOpt)]
#[structopt(about="Manage IVR attr")]
#[derive(Debug)]
pub enum IvrAttrCli {
    #[structopt(about="Add IVR attr")]
    Add {
        #[structopt(short, long, help="IVR id")]
        ivr_id: i32,
        #[structopt(long, help="Attr name")]
        name: String,
        #[structopt(long, help="Attr value")]
        value: String,
    },
    #[structopt(about="Delete IVR attr")]
    Del {
        #[structopt(long, help="Id of the attr to be deleted")]
        id: i32,
    },
    #[structopt(about="List IVR attr")]
    Ls {
        #[structopt(short, long, help="IVR id")]
        ivr_id: i32,
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
        #[structopt(long, help="Greet long")]
        greet_long: String,
        #[structopt(long, help="Greet short")]
        greet_short: String
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
        #[structopt(subcommand, help="manager IVR entries")]
        entry: IvrEntryCli,
    },
    Attr {
        #[structopt(subcommand, help="manager IVR attrs")]
        attr: IvrAttrCli,
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

pub fn print_ivr_entries(ivr_id: i32) {
    let entries = ivr::ivr_entry::list(ivr_id).unwrap();
    let mut table = Ctable::new();

    table.set_titles(row!["id", "ivr id", "digits", "dest extension"]);
    for e in entries {
        table.add_row(
            row![
                e.id,
                e.ivr_id,
                e.digits,
                e.dest_exten
            ]);

    }
    table.print();
}

pub fn exec_ivr_entry_cmd(entry: IvrEntryCli) {
    let domain_id = domain::get_active().unwrap();
    match entry {
        IvrEntryCli::Add {ivr_id, digits, dest_exten} =>  {
            ivr::ivr_entry::add_entry(domain_id, ivr_id, digits, dest_exten).unwrap();
        },
        IvrEntryCli::Del {id} => {
            ivr::ivr_entry::del_entry(id).unwrap();
        },
        IvrEntryCli::Ls {ivr_id} => {
            print_ivr_entries(ivr_id);
        }
    }
}

pub fn print_ivr_attrs(ivr_id: i32) {
    let attrs = ivr::ivr_attrs::list(ivr_id).unwrap();
    let mut table = Ctable::new();

    table.set_titles(row!["id", "ivr id", "name", "value"]);
    for a in attrs {
        table.add_row(row![
            a.id,
            a.ivr_id,
            a.name,
            a.value
        ]);
    }

    table.print();
}

pub fn exec_ivr_attr_cmd(attr: IvrAttrCli) {
    match attr {
        IvrAttrCli::Add {ivr_id, name, value} =>  {
            ivr::ivr_attrs::add_attr(ivr_id, name, value).unwrap();
        },
        IvrAttrCli::Del {id} => {
            ivr::ivr_attrs::del_attr(id).unwrap();
        },
        IvrAttrCli::Ls {ivr_id} => {
            print_ivr_attrs(ivr_id);
        }
    }
}

pub fn exec_ivr_cmd(ivr: IvrCli) {
    let domain_id = domain::get_active().unwrap();
    match ivr {
        IvrCli::Add {name, exten, greet_long, greet_short} => {
            ivr::add(&name, &exten, domain_id, &greet_long, &greet_short).unwrap();
        },
        IvrCli::Del {id} => {
            ivr::del(id).unwrap();
        },
        IvrCli::Ls {} => {
            let ivrs = ivr::list().unwrap();
            print_ivrs(ivrs);
        },
        IvrCli::Entry { entry } => {
            exec_ivr_entry_cmd(entry);
        },
        IvrCli::Attr { attr } => {
            exec_ivr_attr_cmd(attr);
        }
    }
}
