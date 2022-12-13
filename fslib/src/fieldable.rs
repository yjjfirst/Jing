pub trait Fieldable{
    fn fields(&self) -> Vec<&str>;
    fn field_values(&self) -> Vec<String>;
}
