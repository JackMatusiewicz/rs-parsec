mod parser;

pub use crate::parser::*;

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::matcher::Matcher;
    use super::*;

    #[test]
    fn simple_parser_test() {
        let p = matcher_combinators::digits();
        let input = "10";
        let children: Vec<Rc<dyn Matcher<char>>> = vec![Rc::clone(&p), Rc::clone(&p)];
        let two_digit_parser = and_matcher::AndMatcher::new(children);

        two_digit_parser.eval(input).unwrap();
    }
}
