use super::customtable::{Ctable};
use structopt::StructOpt;
use super::domain;
use super::fs_lib::queue::agent;
use super::fs_lib::queue::agent::{Agent};

#[derive(StructOpt)]
#[derive(Debug)]
pub enum AgentCli {
    Ls,
    Add {
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        user_id: i32,
    },
    Del {
        #[structopt(short, long)]
        id: i32
    }
}

pub fn exec_agent_cmd(agent: AgentCli) {
    let domain_id = domain::get_active().unwrap();

    match agent {
        AgentCli::Add {name, user_id} => {
            agent::add(domain_id, user_id, name, 15).unwrap();
        },
        AgentCli::Del {id} => {
            agent::del(id).unwrap();
        },
        AgentCli::Ls => {
            let agents = agent::all().unwrap();
            print_agents(agents);
        }
    }
}

fn print_agents(agents: Vec<Agent>) {
    let mut table = Ctable::new();
    table.set_titles(row!["id", "name", "domain_id", "user_id"]);

    for a in agents {
        table.add_row(row![
            a.id,
            a.name,
            a.domain_id,
            a.user_id
        ]);
    }

    table.print();
}
