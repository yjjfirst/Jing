use structopt::StructOpt;
use crate::customtable::Ctable;
use crate::fs_lib::acl::list as acl_list;
use crate::fs_lib::acl::node;

#[derive(StructOpt)]
#[derive(Debug)]
pub enum NodeCli {
    Ls,
    Add {
        #[structopt(long)]
        list_id: i32,
        #[structopt(long)]
        node_cidr: String,
        #[structopt(long)]
        node_type: String
    },
    Del,
    Edit
}

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
    Node {
        #[structopt(subcommand)]
        node: NodeCli,
    }
}

pub fn exec_node_cmd(node: NodeCli) {
    match node {
        NodeCli::Ls => {
        },
        NodeCli::Add { list_id, node_cidr, node_type } => {
            node::add(list_id, &node_type, &node_cidr).unwrap();
        },
        NodeCli::Del => {
        },
        NodeCli::Edit => {
        },
    }
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
        AclCli::Node {node} => {
            exec_node_cmd(node);
        }
    }
}
