mod parser;

pub use crate::parser::*;

#[cfg(test)]
mod tests {
    use crate::matcher::Matcher;
    use super::*;

    #[test]
    fn simple_parser_test() {
        let p = matcher_combinators::digits();
        let p2 = matcher_combinators::digits();
        let input = "10";
        let children: Vec<Box<dyn Matcher<char>>> = vec![p, p2];
        let two_digit_parser = and_matcher::AndMatcher::new(children);

        two_digit_parser.eval(input).unwrap();
    }
}
