use crate::customtable::Ctable;
use jlib::firewall;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum FirewallCli {
    Ls,
}

pub fn exec_firewall_cmd(command: FirewallCli) {
    match command {
        FirewallCli::Ls => match firewall::list() {
            Ok(rules) => {
                let mut table = Ctable::new();
                table.set_titles(row!["Id", "IP Address", "Action", "Created At"]);

                for rule in rules {
                    table.add_row(row![
                        rule.id, 
                        rule.ip_address, 
                        rule.action, 
                        rule.created_at.format("%Y-%m-%d %H:%M:%S")
                    ]);
                }

                table.print();
            }
            Err(err) => println!("{}", err),
        },
    }
}
