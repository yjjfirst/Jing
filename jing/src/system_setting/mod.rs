use structopt::StructOpt;
use crate::customtable::{Ctable};
use crate::fs_lib::system_setting;

#[derive(StructOpt, Debug)]
pub enum SystemSettingCli {
    Ls,
    Set {
        #[structopt(short, long)]
        section: String,
        #[structopt(short, long)]
        key: String,
        #[structopt(short, long)]
        value: String,
    }
}

pub fn exec_system_setting_cmd(system: SystemSettingCli) {
    match system {
        SystemSettingCli::Ls => {
            let settings = system_setting::list().unwrap();
            print_system_settings(settings);
        },
        SystemSettingCli::Set {section, key, value} => {
            system_setting::update(&section, &key, &value).unwrap();
        }
    }
}

pub fn print_system_settings(settings: Vec<super::fs_lib::system_setting::SystemSetting>) {
    let mut table = Ctable::new();
    table.set_titles(row!["section", "key", "value"]);

    for s in settings {
        table.add_row(row![
            s.setting_section,
            s.setting_key,
            s.setting_value
        ]);
    }

    table.print();
}