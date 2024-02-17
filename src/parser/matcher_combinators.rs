use std::rc::Rc;
use super::char_matcher::CharMatcher;
use super::matcher::Matcher;
use super::or_matcher::OrMatcher;

pub fn digits() -> Rc<dyn Matcher<char>> {
    let digits = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut matchers: Vec<Rc<dyn Matcher<char>>> = vec![];
    for digit in digits {
        let matcher: Rc<dyn Matcher<char>> = Rc::new(CharMatcher::char(digit));
        matchers.push(matcher);
    }

    Rc::new(OrMatcher::new(matchers))
}

pub fn alphanumerics() -> Rc<dyn Matcher<char>> {
    let lower_case: Vec<char> = ('a'..'z').collect();
    let upper_case: Vec<char> = ('A'..'Z').collect();
    let digits = digits();
    let underscore = Rc::new(CharMatcher::char('_'));

    let mut matchers: Vec<Rc<dyn Matcher<char>>> = vec![];
    for &c in lower_case.iter() {
        matchers.push(Rc::new(CharMatcher::char(c)))
    }
    for &c in upper_case.iter() {
        matchers.push(Rc::new(CharMatcher::char(c)))
    }
    matchers.push(digits);
    matchers.push(underscore);

    Rc::new(OrMatcher::new(matchers))
}