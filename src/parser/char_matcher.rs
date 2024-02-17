use super::matcher::Matcher;

pub struct CharMatcher {
    c: char,
    f: Box<dyn for <'a> Fn(&'a str, char) -> Result<(char, &'a str), ()>>
}

impl CharMatcher {
    pub fn char(c: char) -> Self {
        Self {
            c,
            f: Box::new(|s: &str, c: char| {
                if s.as_bytes()[0] == (c as u8) {
                    Ok((c, &s[1..]))
                } else {
                    Err(())
                }
            })
        }
    }
}

impl Matcher<char> for CharMatcher {
    fn eval<'a, 'b>(&'a self, s: &'b str) -> Result<(char, &'b str), ()> {
        (self.f)(s, self.c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn simple_char_parser_success() {
        let p = CharMatcher::char('a');
        let t = "a";
        let result = p.eval(t);
        match result {
            Ok((_,v)) => assert_eq!("", v),
            _ => panic!("Expected a successful parse")
        }
    }

    #[test]
    pub fn simple_char_parser_fail() {
        let p = CharMatcher::char('a');
        let t = "b";
        let result = p.eval(t);
        match result {
            Err(()) => (),
            _ => panic!("Expected a failure to parse")
        }
    }
}