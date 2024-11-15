pub trait Context {
    fn name(&self) -> &'static str;
}