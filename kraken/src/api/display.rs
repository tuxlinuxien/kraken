pub trait ToCSV {
    fn columns() -> Vec<&'static str>;
    fn rows(&self) -> Vec<Vec<String>>;
}
