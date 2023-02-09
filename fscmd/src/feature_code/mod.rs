use super::customtable::{Ctable};
use structopt::StructOpt;
use super::fslib::feature_code::{FeatureCode};
use fslib::printable::Printable;
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
    match feature {
        FeatureCodeCli::Add {digits, action } => {
            FeatureCode::add(&digits, &action).unwrap();
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
