use structopt::StructOpt;
use jlib::user::*;
use crate::customtable::{Ctable};
use crate::print_table;
use jlib::printable::{Printable};

#[derive(StructOpt)]
#[derive(Debug)]
pub enum UserParamCli {
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

pub fn exec_userparam_cmd(param: UserParamCli) {
    match param {
        UserParamCli::Add {user_id, name, value} =>{
            user_param::UserParam::add(user_id, &name, &value).unwrap();
        },
        UserParamCli::Del {id} =>{
            user_param::UserParam::del(id).unwrap();
        },
        UserParamCli::Update {id, name, value} =>{
            user_param::UserParam::update(id, &name, &value).unwrap();
        },
        UserParamCli::Ls {user_id} => {
            let params = get_user_params(user_id).unwrap();
            print_table!(params);
        },
    }
}
