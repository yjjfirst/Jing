mod customtable;
mod ringgroup;
mod ivr;
mod domain;
mod sound_file;
mod sound;
mod conference;
mod queue;
mod agent;
mod tier;
mod extension;
mod user;
mod feature_code;
mod cdr;
mod gateway;
mod acl;

#[macro_use]
extern crate prettytable;
extern crate fs_lib;

use self::customtable::{Ctable};
use structopt::StructOpt;
use self::fs_lib::*;

#[derive(StructOpt)]
#[derive(Debug)]
enum Cli {
    User {
        #[structopt(subcommand, help="manager user.")]
        user: user::UserCli,
    },
    Profile {
        #[structopt(subcommand)]
        profile: ProfileCli,
    },
    Domain {
        #[structopt(subcommand)]
        domain: domain::DomainCli,
    },
    Gateway {
        #[structopt(subcommand)]
        gateway: gateway::GatewayCli,
    },
    Acl {
        #[structopt(subcommand)]
        acl: acl::AclCli,
    },

    Route {
        #[structopt(subcommand)]
        route: RouteCli,
    },

    Cdr {
        #[structopt(subcommand)]
        cdr: cdr::CdrCli,
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
    },
    SoundFile {
        #[structopt(subcommand)]
        soundfile: sound_file::SoundFileCli
    },
    Sound {
        #[structopt(subcommand)]
        sound: sound::SoundCli
    },
    Conference {
        #[structopt(subcommand)]
        conference: conference::ConferenceCli
    },
    Queue {
        #[structopt(subcommand)]
        queue: queue::QueueCli
    },
    Agent {
        #[structopt(subcommand)]
        agent: agent::AgentCli
    },
    Tier {
        #[structopt(subcommand)]
        tier: tier::TierCli
    },
    Extension {
        #[structopt(subcommand)]
        exten: extension::ExtensionCli
    },
    Feature {
        #[structopt(subcommand)]
        feature: feature_code::FeatureCodeCli
    }
}

#[derive(StructOpt)]
#[derive(Debug)]
enum ProfileCli {
    Ls,
    Params {
        #[structopt(short, long)]
        profile_name: String,
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
        dest_extension: String,
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
        #[structopt(long, help="String to prepend to the dialed number.")]
        prepend: String,
        #[structopt(long, help="Prefix for the dialed number.")]
        prefix: i32,
    },
    Del {
        #[structopt(short, long, help="The gateway to delete.")]
        id: i32,
    },
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

    if let Cli::Domain{..} = args {
    } else {
        domain::get_active().expect("Please set FS_ACTIVE_DOMAIN environment variable");
    }

    match args {
        Cli::User { user } => {
            user::exec_user_cmd(user);
        },
        Cli::Profile { profile } => {
            exec_profile_cmd(profile);
        },
        Cli::Domain { domain } => {
            domain::exec_domain_cmd(domain);
        },
        Cli::Gateway { gateway } => {
            gateway::exec_gateway_cmd(gateway);
        },
        Cli::Acl { acl } => {
            acl::exec_acl_cmd(acl);
        },
        Cli::Route { route } => {
            exec_route_cmd(route)
        },
        Cli::Cdr { cdr } => {
            cdr::exec_cdr_cmd(cdr)
        },
        Cli::Voicemail {voicemail} => {
            exec_voicemail_cmd(voicemail)
        },
        Cli::Rg { rg } => {
            ringgroup::exec_rg_cmd(rg)
        },
        Cli::Ivr { ivr } => {
            ivr::exec_ivr_cmd(ivr);
        },
        Cli::SoundFile {soundfile} => {
            sound_file::exec_soundfile_cmd(soundfile);
        },
        Cli::Sound {sound} => {
            sound::exec_sound_cmd(sound);
        },
        Cli::Conference {conference} => {
            conference::exec_conference_cmd(conference);
        },
        Cli::Queue {queue} => {
            queue::exec_queue_cmd(queue);
        },
        Cli::Agent {agent} => {
            agent::exec_agent_cmd(agent);
        },
        Cli::Tier { tier } => {
            tier::exec_tier_cmd(tier);
        },
        Cli::Extension {exten} => {
            extension::exec_extension_cmd(exten);
        },
        Cli::Feature {feature} => {
            feature_code::exec_feature_cmd(feature);
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
            match profile::list() {
                Ok(profiles) => print_profiles(profiles),
                Err(err) => println!("{}", err),
            }
        },

        ProfileCli::Params {profile_name} => {
            let profile = profile::get_profile_by_name(&profile_name).unwrap();
            match profile::get_profile_params(profile.id) {
                Ok(params)  => print_profile_params(params),
                Err(err) => println!("{}", err),
            }
       }
    }
}


fn print_inbounds(routes: Vec<route::inbound_models::InboundRoute>) {

    let mut table = Ctable::new();
    table.set_titles(row!["Id", "Context", "Condition", "Dest"]);

    for r in routes {
        table.add_row(row![r.id, r.context, r.condition, r.dest_extension]);
    }

    table.print();

}

fn exec_inbound_cmd(inbound: InRouteCli) {
    match inbound {
        InRouteCli::Add {context, condition, dest_extension} => {
            route::inbound::add(&context, &condition, &dest_extension)
                .unwrap_or_else(|err| println!("{}",err));
        },
        InRouteCli::Del {id} => {
            route::inbound::del(id)
                .unwrap_or_else(|err| println!("{}",err));
        }
        InRouteCli::Ls {} => {
            match route::inbound::list() {
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
        OutRouteCli::Add {gateway_id, priority, condition, prepend, prefix} => {
            route::outbound::add(gateway_id, priority, &condition, &prepend, prefix)
                .unwrap_or_else(|err| println!("{}",err));
        },

        OutRouteCli::Del {id} => {
            route::outbound::del(id)
                .unwrap_or_else(|err| println!("{}",err));
        },

        OutRouteCli::Ls {} => {
            match route::outbound::list() {
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
