use super::customtable::{Ctable};
use structopt::StructOpt;
use super::domain;
use fslib::user;

#[derive(StructOpt)]
#[derive(Debug)]
pub enum UserCli {
    Add {
        #[structopt(short, long)]
        user_id: String,
        #[structopt(short, long)]
        password: String
    },

    Del {
        #[structopt(short, long)]
        user_id: String,
    },

    Ls,
}

pub fn exec_user_cmd(user: UserCli) {
    let domain_id = domain::get_active().unwrap();
    match user {
        UserCli::Add {user_id, password} => {
            user::add_user(
                domain_id,
                &user_id,
                &password).unwrap_or_else(|err| println!("{}",err));
        }

        UserCli::Ls => {
            match user::users_within_domain(domain_id) {
                Ok(users) => exec_user_ls_cmd(users),
                Err(err) => println!("{}", err),
            }
        }

        UserCli::Del { user_id }=> {
            user::del_user(&user_id)
                .unwrap_or_else(|err| println!("{}",err));
        }
    }
}


fn exec_user_ls_cmd(users: Vec<(i32, String, String, String)>) {
    let mut table = Ctable::new();

    table.set_titles(row!["Id", "User_id", "Password", "Domain"]);
    for u in users {
        table.add_row(row![u.0, u.1, u.2, u.3]);
    }

    table.print();
}
