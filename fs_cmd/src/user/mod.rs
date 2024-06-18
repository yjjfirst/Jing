pub mod user_param;
pub mod user_variable;

use super::customtable::{Ctable};
use structopt::StructOpt;
use super::domain;
use fs_lib::user;
use fs_lib::user::models::{User};
use fs_lib::printable::Printable;
use crate::print_table;

#[derive(StructOpt)]
#[derive(Debug)]
pub enum UserCli {
    Add {
        #[structopt(short, long)]
        user_id: String,
    },

    Del {
        #[structopt(short, long)]
        id: i32,
    },

    Ls,
    Param {
        #[structopt(subcommand)]
        param: user_param::UserParamCli,
    },
    Var {
        #[structopt(subcommand)]
        var: user_variable::UserVariableCli,
    }
}

pub fn exec_user_cmd(user: UserCli) {
    let domain_id = domain::get_active().unwrap();
    match user {
        UserCli::Add {user_id} => {
            user::add_user(
                domain_id,
                &user_id).unwrap_or_else(|err| println!("{}",err));
        }

        UserCli::Ls => {
            match user::users_within(domain_id) {
                Ok(users) => exec_user_ls_cmd(users),
                Err(err) => println!("{}", err),
            }
        }

        UserCli::Del { id }=> {
            user::del_user(id)
                .unwrap_or_else(|err| println!("{}",err));
        }

        UserCli::Param {param} =>{
            user_param::exec_userparam_cmd(param);
        }

        UserCli::Var {var} =>{
            user_variable::exec_user_var_cmd(var);
        }
    }
}



fn exec_user_ls_cmd(users: Vec<User>) {
    print_table!(users);
}
