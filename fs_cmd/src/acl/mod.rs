use structopt::StructOpt;
use crate::customtable::Ctable;
use crate::fs_lib::acl::list as acl_list;

#[derive(StructOpt)]
#[derive(Debug)]
pub enum AclCli {
    Ls,
    Add {
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        default: String,
    },
    Del {
        #[structopt(short, long)]
        id: i32,
    },
    Edit {
        #[structopt(short, long)]
        id: i32,
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        default: String,
    },
}

pub fn exec_acl_cmd(acl: AclCli) {
    match acl {
        AclCli::Add { name, default } => {
            match acl_list::add(&name, &default) {
                Ok(id) => println!("Inserted acl list id {}", id),
                Err(err) => println!("{}", err),
            }
        }
        AclCli::Del { id } => {
            acl_list::del(id).unwrap_or_else(|err| println!("{}", err));
        }
        AclCli::Edit { id, name, default } => {
            acl_list::edit(id, &name, &default).unwrap_or_else(|err| println!("{}", err));
        }
        AclCli::Ls => {
            match acl_list::list() {
                Ok(lists) => {
                    let mut table = Ctable::new();
                    table.set_titles(row!["Id", "Name", "Default"]);
                    for l in lists {
                        table.add_row(row![l.id, l.acl_name, l.acl_default]);
                    }
                    table.print();
                }
                Err(err) => println!("{}", err),
            }
        }
    }
}
