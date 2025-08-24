use crate::series_parser::series_parser;
use chumsky::prelude::*;

pub fn string_parser<'a>(
    word: String,
) -> impl Parser<'a, &'a str, Vec<char>, extra::Err<Simple<'a, char>>> + Clone + 'a {
    if word.is_empty() {
        // Accept only empty input
        return chumsky::primitive::end().to(vec![]).boxed();
    }
    let char_parsers = word.chars().map(|c| just(c)).collect::<Vec<_>>();
    series_parser(char_parsers).boxed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn test_string_parser_basic() {
        let parser = string_parser("abc".to_string());
        let result = parser.parse("abc").into_result();
        assert_eq!(result, Ok(vec!['a', 'b', 'c']));
    }

    #[test]
    fn test_string_parser_empty() {
        let parser = string_parser("".to_string());
        let result = parser.parse("").into_result();
        assert_eq!(result, Ok(vec![]));
    }

    #[test]
    fn test_string_parser_partial_match() {
        let parser = string_parser("abc".to_string());
        let result = parser.parse("ab").into_result();
        assert!(result.is_err());
    }

    #[test]
    fn test_string_parser_extra_input() {
        let parser = string_parser("abc".to_string());
        let result = parser.parse("abcd").into_result();
        assert!(result.is_err());
    }
}
