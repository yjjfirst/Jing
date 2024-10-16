use super::customtable::{Ctable};
use structopt::StructOpt;
use fs_lib::*;
use super::domain;

#[derive(StructOpt)]
#[derive(Debug)]
pub enum ExtensionCli {
    Ls
}

pub fn exec_extension_cmd(exten: ExtensionCli) {
    let domain_id = domain::get_active().unwrap();
    match exten {
        ExtensionCli::Ls => {
            let extensions = extension::list(domain_id).unwrap();
            let mut table = Ctable::new();

            table.set_titles(row!["id", "exten", "type", "domain_id"]);

            for e in extensions {
                table.add_row(
                    row![
                        e.id,
                        e.exten,
                        e.exten_type,
                        e.domain_id
                    ])
            }

            table.print();
        }
    }
}
