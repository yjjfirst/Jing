mod customtable;
mod ringgroup;
mod ivr;

#[macro_use]
extern crate prettytable;
extern crate fslib;

use self::customtable::{Ctable};
use structopt::StructOpt;
use self::fslib::*;

#[derive(StructOpt)]
#[derive(Debug)]
enum Cli {
    User {
        #[structopt(subcommand, help="manager user.")]
        user: UserCli,
    },
    Profile {
        #[structopt(subcommand)]
        profile: ProfileCli,
    },
    Domain {
        #[structopt(subcommand)]
        domain: DomainCli,
    },
    Gateway {
        #[structopt(subcommand)]
        gateway: GatewayCli,
    },

    Route {
        #[structopt(subcommand)]
        route: RouteCli,
    },

    Cdr {
        #[structopt(subcommand)]
        cdr: CdrCli,
    },

    Voicemail {
        #[structopt(subcommand)]
        voicemail: VoicemailCli
    },

    Rg {
        #[structopt(subcommand)]
        rg: ringgroup::RgCli
    },

    Ivr {
        #[structopt(subcommand)]
        ivr: ivr::IvrCli
    }
}

#[derive(StructOpt)]
#[derive(Debug)]
enum UserCli {
    Add {
        #[structopt(short, long)]
        domain: String,
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

#[derive(StructOpt)]
#[derive(Debug)]
enum ProfileCli {
    Ls,
    List {
        #[structopt(short, long)]
        profile: String,
    }
}

#[derive(StructOpt)]
#[derive(Debug)]
enum DomainCli {
    Ls,
    Add {
        #[structopt(short, long)]
        name: String,
    },
    Del {
        #[structopt(short, long)]
        id: i32,
    },
}
#[derive(StructOpt)]
#[derive(Debug)]
enum GatewayCli {
    Ls,
    Add {
        #[structopt(long)]
        profile: i32,
        #[structopt(short, long)]
        name: String,
        #[structopt(long)]
        proxy: String,
        #[structopt(short, long)]
        register: String,
        #[structopt(short, long)]
        username: String,
        #[structopt(long)]
        password: String
    },
    Del {
        #[structopt(short, long)]
        id: i32
    }

}

#[derive(StructOpt)]
#[derive(Debug)]
enum RouteCli {
    In {
        #[structopt(subcommand)]
        inbound: InRouteCli,
    },
    Out {
        #[structopt(subcommand)]
        outbound: OutRouteCli,
    }
}

#[derive(StructOpt)]
#[derive(Debug)]
enum InRouteCli {
    Add {
        #[structopt(short = "x", long)]
        context: String,
        #[structopt(short, long)]
        condition: String,
        #[structopt(short = "t", long)]
        dest_type: String,
        #[structopt(short, long)]
        dest: i32,
    },
    Del {
        #[structopt(short, long)]
        id: i32,
    },
    Ls
}

#[derive(StructOpt)]
#[derive(Debug)]
enum OutRouteCli {
    Add {
        #[structopt(short, long, help="The gateway to place the call.")]
        gateway_id: i32,
        #[structopt(short, long, help="Priority of this route.")]
        priority: i32,
        #[structopt(short, long, help="Regular expression to match the dialed number.")]
        condition: String,
    },
    Del {
        #[structopt(short, long, help="The gateway to delete.")]
        id: i32,
    },
    Ls
}

#[derive(StructOpt)]
#[derive(Debug)]
enum CdrCli {
    Ls
}

#[derive(StructOpt)]
#[derive(Debug)]
enum VoicemailCli {
    Add  {
        #[structopt(short, long)]
        user_id: i32,
        #[structopt(short, long)]
        password: String,
        #[structopt(short, long)]
        email: Option<String>
    },
    Del {
        #[structopt(short, long)]
        id: i32
    },
    Ls
}

fn main() {
    let args = Cli::from_args();

    match args {
        Cli::User { user } => {
            exec_user_cmd(user);
        },
        Cli::Profile { profile } => {
            exec_profile_cmd(profile);
        },
        Cli::Domain { domain } => {
            exec_domain_cmd(domain);
        },
        Cli::Gateway { gateway } => {
            exec_gateway_cmd(gateway);
        },
        Cli::Route { route } => {
            exec_route_cmd(route)
        },
        Cli::Cdr { cdr } => {
            exec_cdr_cmd(cdr)
        },
        Cli::Voicemail {voicemail} => {
            exec_voicemail_cmd(voicemail)
        },
        Cli::Rg { rg } => {
            ringgroup::exec_rg_cmd(rg)
        },
        Cli::Ivr { ivr } => {
            ivr::exec_ivr_cmd(ivr);
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

fn exec_user_cmd(user: UserCli) {
    match user {
        UserCli::Add {domain, user_id, password} => {
            let domain_id: i32 = domain.parse().expect("Domain Id is wrong.");
            user::add_user(
                domain_id,
                &user_id,
                &password,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None).unwrap_or_else(|err| println!("{}",err));


        }

        UserCli::Ls => {
            match user::all_users_with_domain() {
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

fn print_profiles(profiles:  Vec<profile::models::Profile>) {
    let mut table = Ctable::new();

    table.set_titles(row!["Id", "Name"]);
    for p in profiles {
        table.add_row(row![p.id, p.name]);
    }

    table.print();

}

fn print_profile_params(params: Vec<profile::models::ProfileParam>) {
    let mut table = Ctable::new();

    table.set_titles(row!["Name", "Value"]);

    for p in params {
        table.add_row(row![p.name, p.value]);
    }

    table.print();

}

fn exec_profile_cmd(profile: ProfileCli) {
    match profile {
        ProfileCli::Ls => {
            match profile::all_profiles() {
                Ok(profiles) => print_profiles(profiles),
                Err(err) => println!("{}", err),
            }
        },

        ProfileCli::List {profile} => {
            match profile::profile_params(profile) {
                Ok(params)  => print_profile_params(params),
                Err(err) => println!("{}", err),
            }
       }
    }
}

fn print_domains(domains: Vec<domain::models::Domain>) {
    let mut table = Ctable::new();

    table.set_titles(row!["Id", "Domain Name"]);
    for d in domains {
        table.add_row(row![d.id, d.domain_name]);
    }

    table.print();

}

fn exec_domain_cmd(domain: DomainCli) {
    match domain {
        DomainCli::Add { name }=> {
            domain::add_domain(&name)
                .unwrap_or_else(|err| println!("{}", err));
        },

        DomainCli::Del { id } => {
            domain::del_domain(id)
                .unwrap_or_else(|err| println!("{}", err));
        },

        DomainCli::Ls => {
            match domain::list_domains() {
                Ok(domains) => print_domains(domains),
                Err(err) => println!("{}", err),
            }
        }
    }
}

fn print_gateways(gateways: Vec<gateway::models::Gateway>)  {

    let mut table = Ctable::new();

    table.set_titles(row!["Id", "Profile_id", "name", "proxy", "register", "username", "password"]);

    for g in gateways
    {
        let username = g.username.unwrap_or("".to_string());
        let password = g.password.unwrap_or("".to_string());

        table.add_row(row![g.id, g.profile_id, g.gateway_name, g.proxy, g.register, username, password]);
    }

    table.print();

}

fn exec_gateway_cmd(gateway: GatewayCli) {
    match gateway {
        GatewayCli::Add {profile, name, proxy, register, username, password} => {
            gateway::add_gateway(
                profile,
                name,
                proxy,
                register,
                Some(username),
                Some(password)
            ).unwrap_or_else(|err| println!("{}",err));

        },

        GatewayCli::Del {id} => {
            gateway::del_gateway(id)
                .unwrap_or_else(|err| println!("{}",err));
        },

        GatewayCli::Ls => {
            match gateway::all_gateways() {
                Ok(gateways) => print_gateways(gateways),
                Err(err) => println!("{}", err),
            }
        }
    }
}

fn print_inbounds(routes: Vec<route::inbound_models::InboundRoute>) {

    let mut table = Ctable::new();
    table.set_titles(row!["Id", "Context", "Condition", "Dest Type", "Dest"]);

    for r in routes {
        table.add_row(row![r.id, r.context, r.condition,r.dest_type, r.dest]);
    }

    table.print();

}

fn exec_inbound_cmd(inbound: InRouteCli) {
    match inbound {
        InRouteCli::Add {context, condition, dest_type, dest} => {
            route::add_inboud(&context, &condition, &dest_type, dest)
                .unwrap_or_else(|err| println!("{}",err));
        },
        InRouteCli::Del {id} => {
            route::del_inbound(id)
                .unwrap_or_else(|err| println!("{}",err));
        }
        InRouteCli::Ls {} => {
            match route::all_inbound() {
                Ok(routes) => print_inbounds(routes),
                Err(err) => println!("{}", err)
            }
        }
    }
}

fn print_outbounds(routes: Vec<route::outbound_models::OutboundRoute>) {
    let mut table = Ctable::new();

    table.set_titles(row!["Id", "Gateway Id", "Priority", "Condition"]);
    for r in routes {
        table.add_row(row![r.id, r.gateway_id, r.priority, r.condition]);
    }

    table.print();

}

fn exec_outbound_cmd(outbound: OutRouteCli) {
    match outbound {
        OutRouteCli::Add {gateway_id, priority, condition} => {
            route::add_outbound(gateway_id, priority, &condition)
                .unwrap_or_else(|err| println!("{}",err));
        },

        OutRouteCli::Del {id} => {
            route::del_outbound(id)
                .unwrap_or_else(|err| println!("{}",err));
        },

        OutRouteCli::Ls {} => {
            match route::all_outbounds() {
                Ok( routes) => print_outbounds(routes),
                Err(err) => println!("{}", err),
            }
        }
    }
}

fn exec_route_cmd(route: RouteCli) {
    match route {
        RouteCli::In {inbound} => {
            exec_inbound_cmd(inbound);
        },
        RouteCli::Out {outbound } => {
            exec_outbound_cmd(outbound);
        }
    }
}

fn exec_cdr_cmd(cdr: CdrCli) {
    match cdr {
        CdrCli::Ls {} => {
            let cdrs = cdr::all_cdrs()
                .unwrap();
            let mut table = Ctable::new();

            table.set_titles(row!["a_caller_id", "a_dest", "b_caller_id", "b_dest", "Duration"]);
            for cdr in cdrs {
                table.add_row(
                    row![cdr.a_caller_id,
                         cdr.a_dest,
                         cdr.b_caller_id.unwrap_or("".to_string()),
                         cdr.b_dest.unwrap_or("".to_string()),
                         cdr.duration
                    ])
            }

            table.print();

        }
    }
}

fn print_voicemails(voicemails: Vec<voicemail::models::Voicemail>) {
    let mut table = Ctable::new();
    table.set_titles(row!["id", "user_id", "password", "email"]);
    for vm in voicemails {
        table.add_row(
            row![vm.id,
                 vm.user_id,
                 vm.password,
                 vm.email.unwrap_or("".to_string())
            ])
    }

    table.print();

}

fn exec_voicemail_cmd(voicemail: VoicemailCli) {
    match voicemail {
        VoicemailCli::Add {user_id, password, email}=> {
            voicemail::add_voicemail(user_id, password, email)
                .expect("Add new voicemail failed");
        }
        VoicemailCli::Del {id} => {
            voicemail::del_voicemail(id)
                .expect("Delete voicemail failed");
        }
        VoicemailCli::Ls => {
            let voicemails = voicemail::all_voicemails();
            match voicemails {
                Ok(voicemails) => {
                    print_voicemails(voicemails);
                }
                Err(e) => {
                    print!("{}", e);
                }
            }
        }
    }
}
