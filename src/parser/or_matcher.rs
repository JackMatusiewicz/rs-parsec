use super::matcher::Matcher;

pub struct OrMatcher {
    matchers: Vec<Box<dyn Matcher>>
}

impl OrMatcher {
    pub fn new(matchers: Vec<Box<dyn Matcher>>) -> Self {
        Self {
            matchers
        }
    }
}

impl Matcher for OrMatcher {
    fn eval<'a, 'b>(&'a self, s: &'b str) -> Result<&'b str, ()> {
        for i in 0 .. self.matchers.len() {
            let m: &Box<dyn Matcher> = &(self.matchers[i]);
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
        let om: OrMatcher = OrMatcher::new(vec![a, b]);
        om.eval("a").unwrap();
        om.eval("b").unwrap();
    }

    #[test]
    #[should_panic]
    pub fn simple_or_test_fail() {
        let a = Box::new(CharMatcher::char('a'));
        let b = Box::new(CharMatcher::char('b'));
        let om: OrMatcher = OrMatcher::new(vec![a, b]);
        om.eval("c").unwrap();
    }
}