use super::customtable::{Ctable};
use structopt::StructOpt;
use super::jlib::feature_code::{FeatureCode};
use super::domain;
use jlib::printable::Printable;
use crate::print_table;

#[derive(StructOpt)]
#[derive(Debug)]
pub enum FeatureCodeCli {
    Ls,
    Add {
        #[structopt(short, long)]
        digits: String,
        #[structopt(short, long)]
        action: String
    },
    Del {
        #[structopt(short, long)]
        id: i32
    },
    Update {
        #[structopt(short, long)]
        id: i32,
        #[structopt(short, long)]
        digits: String,
        #[structopt(short, long)]
        action: String
    }
}

pub fn exec_feature_cmd(feature: FeatureCodeCli) {
    let domain_id = domain::get_active().unwrap();
    match feature {
        FeatureCodeCli::Add {digits, action } => {
            FeatureCode::add(domain_id, &digits, &action).unwrap();
        },
        FeatureCodeCli::Del { id } => {
            FeatureCode::del(id).unwrap();
        },
        FeatureCodeCli::Update {id, digits, action} => {
            FeatureCode::update(id, &digits, &action).unwrap();
        },
        FeatureCodeCli::Ls => {
            let features = FeatureCode::ls().unwrap();
            print_table!(features);
        }
    }
}
