use super::customtable::{Ctable};
use structopt::StructOpt;
use fs_lib::cdr;

#[derive(StructOpt)]
#[derive(Debug)]
pub enum CdrCli {
    Ls
}

pub fn exec_cdr_cmd(cdr: CdrCli) {
    match cdr {
        CdrCli::Ls {} => {
            let cdrs = cdr::list()
                .unwrap();
            let mut table = Ctable::new();

            table.set_titles(row!["caller_name",
                                  "caller_number",
                                  "dst_number",
                                  "start",
                                  "end",
                                  "Duration",
                                  "Billsec"
            ]);
            for cdr in cdrs {
                table.add_row(
                    row![cdr.caller_id_number.unwrap(),
                         cdr.caller_id_name.unwrap(),
                         cdr.destination_number,
                         cdr.start_stamp,
                         cdr.end_stamp,
                         cdr.duration,
                         cdr.billsec
                    ])
            }

            table.print();

        }
    }
}
