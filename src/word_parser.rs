use chumsky::prelude::*;

pub fn word_parser<'a>(
    groups: Vec<(char, Vec<String>)>,
) -> impl Parser<'a, &'a str, Vec<(char, String)>, extra::Err<Simple<'a, char>>> {
    groups
        .into_iter()
        .rev()
        .fold(end().to(Vec::new()).boxed(), |rest, (kanji, alts)| {
            // Sort so longer readings are attempted first
            let mut sorted = alts.clone();
            sorted.sort_by_key(|s| std::cmp::Reverse(s.chars().count()));

            let choice_for_this_kanji = choice(
                sorted
                    .into_iter()
                    .map(|r| {
                        // Gate: only succeed with r if r + rest parses
                        let gate = just(r.clone()).ignore_then(rest.clone());
                        just(r.clone())
                            .and_is(gate) // lookahead
                            .map(move |_| (kanji, r.clone())) // keep the mapping!
                            .boxed()
                    })
                    .collect::<Vec<_>>(),
            );

            choice_for_this_kanji
                .then(rest)
                .map(|(this, mut rest_vec)| {
                    rest_vec.insert(0, this);
                    rest_vec
                })
                .boxed()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_parser() {
        let input = "12";
        let pairs = vec![
            ('1', vec!["a".to_string(), "ab".to_string()]),
            ('2', vec!["bc".to_string()]),
        ];
        let parser = word_parser(pairs);
        let result = parser.parse(input).into_result();
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result == vec![('1', "a".to_string()), ('2', "bc".to_string())]);
    }
}
