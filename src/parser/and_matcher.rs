use super::matcher::Matcher;

pub struct AndMatcher {
    matchers: Vec<Box<dyn Matcher>>
}

impl AndMatcher {
    pub fn new(matchers: Vec<Box<dyn Matcher>>) -> Self {
        Self {
            matchers
        }
    }
}

impl Matcher for AndMatcher {
    fn eval<'a, 'b>(&'a self, s: &'b str) -> Result<&'b str, ()> {
        let mut remaining_string = s;
        for i in 0 .. self.matchers.len() {
            let m: &Box<dyn Matcher> = &(self.matchers[i]);
            match m.eval(remaining_string) {
                Ok(s) => remaining_string = s,
                Err(()) => return Err(())
            }
        }

        Ok(remaining_string)
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
        let om: AndMatcher = AndMatcher::new(vec![a, b]);
        om.eval("ab").unwrap();
    }

    #[test]
    #[should_panic]
    pub fn simple_or_test_fail() {
        let a = Box::new(CharMatcher::char('a'));
        let b = Box::new(CharMatcher::char('b'));
        let om: AndMatcher = AndMatcher::new(vec![a, b]);
        om.eval("c").unwrap();
    }
}