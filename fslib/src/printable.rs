pub trait Printable {
    fn titles(&self) -> Vec<&str>;
    fn row(&self) -> Vec<String>;
}
