use prettytable::{Table, format, Row, Cell};

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

    pub fn print_table(titles: &Vec<&str>, values: &Vec<Vec<String>>) {

        let mut table = Ctable::new();
        let title_cells: Vec<Cell> = titles.into_iter().map(|f| Cell::new(f)).collect();
        let title_row = Row::new(title_cells);
        table.set_titles(title_row);

        for vs in values {
            let cells: Vec<Cell> = vs.into_iter().map(|f| Cell::new(&f)).collect();
            let row = Row::new(cells);
            table.add_row(row);
        }

        table.print();
    }
}
