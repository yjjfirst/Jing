 use super::customtable::{Ctable};
use structopt::StructOpt;
use super::domain;
use super::jlib::callcenter::queue::{Queue};
use super::jlib::callcenter::queue;

#[derive(StructOpt)]
#[derive(Debug)]
pub enum QueueCli {
    Ls,
    Add {
        #[structopt(short, long)]
        name: String,
        #[structopt(short, long)]
        exten: String,
    },
    Del {
        #[structopt(short, long)]
        id: i32
    }
}


pub fn print_queues(queues: Vec<Queue>) {
    let mut table = Ctable::new();
    table.set_titles(row!["id", "exten", "name"]);

    for q in queues {
        table.add_row(row![
            q.id,
            q.exten,
            q.name
        ]);
    }

    table.print();
}

pub fn exec_queue_cmd(queue: QueueCli) {
    let domain_id = domain::get_active().unwrap();

    match queue {
        QueueCli::Add {exten, name} => {
            queue::add(domain_id, exten, name).unwrap();
        },
        QueueCli::Del {id} => {
            queue::del(id).unwrap();
        },
        QueueCli::Ls => {
            let queues = queue::list(domain_id).unwrap();
            print_queues(queues);
        }
    }
}
