pub trait Matcher {
    fn eval<'a, 'b>(&'a self, s: &'b str) -> Result<&'b str, ()>;
}