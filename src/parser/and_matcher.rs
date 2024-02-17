use std::rc::Rc;
use super::matcher::Matcher;

pub struct AndMatcher<T> {
    matchers: Vec<Rc<dyn Matcher<T>>>
}

impl<T> AndMatcher<T> {
    pub fn new(matchers: Vec<Rc<dyn Matcher<T>>>) -> Self {
        Self {
            matchers
        }
    }
}

impl<T> Matcher<Vec<T>> for AndMatcher<T> {
    fn eval<'a, 'b>(&'a self, s: &'b str) -> Result<(Vec<T>, &'b str), ()> {
        let mut collected = vec![];
        let mut remaining_string = s;
        for m in self.matchers.iter() {
            match m.eval(remaining_string) {
                Ok((a,s)) => {
                    collected.push(a);
                    remaining_string = s;
                },
                Err(()) => return Err(())
            }
        }

        Ok((collected, remaining_string))
    }
}

#[cfg(test)]
mod test {
    use crate::parser::char_matcher::CharMatcher;
    use super::*;

    #[test]
    pub fn simple_or_test_success() {
        let a: Rc<dyn Matcher<char>> = Rc::new(CharMatcher::char('a'));
        let b: Rc<dyn Matcher<char>> = Rc::new(CharMatcher::char('b'));
        let om: AndMatcher<char> = AndMatcher::new(vec![a, b]);
        assert_eq!(Ok((vec!['a','b'], "")), om.eval("ab"));
    }

    #[test]
    #[should_panic]
    pub fn simple_or_test_fail() {
        let a: Rc<dyn Matcher<char>> = Rc::new(CharMatcher::char('a'));
        let b: Rc<dyn Matcher<char>> = Rc::new(CharMatcher::char('b'));
        let om: AndMatcher<char> = AndMatcher::new(vec![a, b]);
        om.eval("c").unwrap();
    }
}