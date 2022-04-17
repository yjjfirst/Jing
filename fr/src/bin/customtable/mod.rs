use prettytable::{Table, format, Row};

pub struct Ctable {
    table: Table,
}

impl Ctable {
    pub fn new() -> Ctable {
        let mut t = Ctable { table: Table::new()};
        t.table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        t
    }

    pub fn print(&self) {
        if self.table.len() > 0 {
            self.table.printstd();
        }
    }

    pub fn set_titles(&mut self, titles: Row) {
        self.table.set_titles(titles);
    }

    pub fn add_row(&mut self, row: Row) {
        self.table.add_row(row);
    }
}

