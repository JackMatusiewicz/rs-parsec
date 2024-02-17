use super::matcher::Matcher;

pub struct OrMatcher<T> {
    matchers: Vec<Box<dyn Matcher<T>>>
}

impl<T> OrMatcher<T> {
    pub fn new(matchers: Vec<Box<dyn Matcher<T>>>) -> Self {
        Self {
            matchers
        }
    }
}

impl<T> Matcher<T> for OrMatcher<T> {
    fn eval<'a, 'b>(&'a self, s: &'b str) -> Result<(T, &'b str), ()> {
        for m in self.matchers.iter() {
            match m.eval(s) {
                Ok(s) => return Ok(s),
                _ => ()
            }
        }

        Err(())
    }
}

#[cfg(test)]
mod test {
    use crate::parser::char_matcher::CharMatcher;
    use super::*;

    #[test]
    pub fn simple_or_test_success() {
        let a = Box::new(CharMatcher::char('a'));
        let b = Box::new(CharMatcher::char('b'));
        let om: OrMatcher<char> = OrMatcher::new(vec![a, b]);
        om.eval("a").unwrap();
        om.eval("b").unwrap();
    }

    #[test]
    #[should_panic]
    pub fn simple_or_test_fail() {
        let a = Box::new(CharMatcher::char('a'));
        let b = Box::new(CharMatcher::char('b'));
        let om: OrMatcher<char> = OrMatcher::new(vec![a, b]);
        om.eval("c").unwrap();
    }
}