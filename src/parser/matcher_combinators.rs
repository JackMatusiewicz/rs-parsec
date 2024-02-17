use super::char_matcher::CharMatcher;
use super::matcher::Matcher;
use super::or_matcher::OrMatcher;

pub fn digits() -> Box<dyn Matcher> {
    let digits = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut matchers: Vec<Box<dyn Matcher>> = vec![];
    for digit in digits {
        let matcher: Box<dyn Matcher> = Box::new(CharMatcher::char(digit));
        matchers.push(matcher);
    }

    Box::new(OrMatcher::new(matchers))
}

pub fn alphanumerics() -> Box<dyn Matcher> {
    let lower_case: Vec<char> = ('a'..'z').collect();
    let upper_case: Vec<char> = ('A'..'Z').collect();
    let digits = digits();
    let underscore = Box::new(CharMatcher::char('_'));

    let mut matchers: Vec<Box<dyn Matcher>> = vec![];
    for &c in lower_case.iter() {
        matchers.push(Box::new(CharMatcher::char(c)))
    }
    for &c in upper_case.iter() {
        matchers.push(Box::new(CharMatcher::char(c)))
    }
    matchers.push(digits);
    matchers.push(underscore);

    Box::new(OrMatcher::new(matchers))
}