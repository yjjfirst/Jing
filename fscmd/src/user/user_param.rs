use structopt::StructOpt;
use fslib::user::*;

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
        }
    }
}
