pub trait Matcher<T> {
    fn eval<'a, 'b>(&'a self, s: &'b str) -> Result<(T, &'b str), ()>;
}