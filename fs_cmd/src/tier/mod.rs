use super::customtable::{Ctable};
use structopt::StructOpt;
use fs_lib::callcenter::tier;

#[derive(StructOpt)]
#[derive(Debug)]
pub enum TierCli {
    Ls {
        #[structopt(short, long)]
        queue_id: Option<i32>
    },
    Add {
        #[structopt(short, long)]
        agent_id: i32,
        #[structopt(short, long)]
        queue_id: i32,
        #[structopt(short, long)]
        level: i32,
        #[structopt(short, long)]
        position: i32,
    },
    Del {
        #[structopt(short, long)]
        id: i32,
    }
}

pub fn exec_tier_cmd(tier: TierCli) {
    match tier {
        TierCli::Add {agent_id, queue_id, level, position} => {
            tier::add(agent_id, queue_id, level, position).unwrap();
        },
        TierCli::Del { id } => {
            tier::del(id).unwrap();
        },
        TierCli::Ls {queue_id}=> {
            let tiers = if let Some(id) = queue_id {
                tier::list(id).unwrap()
            } else {
                tier::list(0).unwrap()
            };

            print_tiers(tiers);
        }
    }
}

fn print_tiers(tiers: Vec<tier::Tier>) {
    let mut table = Ctable::new();
    table.set_titles(row!["id", "agent_id", "queue_id", "level", "position"]);

    for t in tiers {
        table.add_row(row![
            t.id,
            t.agent_id,
            t.queue_id,
            t.level,
            t.position
        ])
    }

    table.print();
}
