use chumsky::prelude::*;

pub fn series_parser<'a, T: Clone + 'a>(
    parsers: Vec<impl Parser<'a, &'a str, T, extra::Err<Simple<'a, char>>> + Clone + 'a>,
) -> impl Parser<'a, &'a str, Vec<T>, extra::Err<Simple<'a, char>>> + Clone + 'a {
    if parsers.is_empty() {
        return chumsky::primitive::end().to(vec![]).boxed();
    }
    if parsers.len() == 1 {
        return parsers[0].clone().map(|v| vec![v]).boxed();
    }

    let parser_0 = parsers[0].clone().boxed();
    let parser_1 = parsers[1].clone().boxed();
    let mut parser = parser_0
        .then(parser_1.clone())
        .map(|(v1, v2)| {
            let mut v = vec![v1];
            v.push(v2);
            v
        })
        .boxed();

    for p in parsers.into_iter().skip(2) {
        parser = parser
            .then(p.clone())
            .map(|(v1, v2)| {
                let mut v = v1;
                v.push(v2);
                v
            })
            .boxed();
    }

    parser
}
