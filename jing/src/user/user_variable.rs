use structopt::StructOpt;
use jlib::user::*;
use crate::customtable::{Ctable};
use crate::print_table;
use jlib::printable::{Printable};

#[derive(StructOpt)]
#[derive(Debug)]
pub enum UserVariableCli {
    Add {
        #[structopt(short, long)]
        user_id: i32,
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        value: String
    },
    Del {
        #[structopt(short, long)]
        id: i32
    },
    Update {
        #[structopt(short, long)]
        id: i32,
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        value: String
    },
    Ls {
        #[structopt(short, long)]
        user_id: i32,
    }

}

pub fn exec_user_var_cmd(var: UserVariableCli) {
    match var {
        UserVariableCli::Add {user_id, name, value}=> {
            user_variable::UserVariable::add(user_id, &name, &value).unwrap();
        },
        UserVariableCli::Del {id}=>{
            user_variable::UserVariable::del(id).unwrap();
        },
        UserVariableCli::Update {id, name, value} => {
            user_variable::UserVariable::update(id, &name, &value).unwrap();
        },
        UserVariableCli::Ls {user_id} => {
            let vars = get_user_vars(user_id).unwrap();
            print_table!(vars);
        }
    }
}
