use chumsky::prelude::*;
use std::collections::HashMap;
use wana_kana;

mod series_parser;
mod string_parser;

use series_parser::series_parser;
use string_parser::string_parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Furigana {
    Kanji { character: char, reading: String },
    Other(char),
}
impl Furigana {
    pub fn to_writing(&self) -> String {
        match self {
            Furigana::Kanji { character, .. } => character.to_string(),
            Furigana::Other(char) => char.to_string(),
        }
    }

    pub fn to_reading(&self) -> String {
        match self {
            Furigana::Kanji { reading, .. } => reading.clone(),
            Furigana::Other(char) => char.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuriganaString(Vec<Furigana>);
impl FuriganaString {
    pub fn to_writing(&self) -> String {
        self.0.iter().map(|f| f.to_writing()).collect::<String>()
    }

    pub fn to_reading(&self) -> String {
        self.0.iter().map(|f| f.to_reading()).collect::<String>()
    }

    pub fn to_vec(&self) -> Vec<Furigana> {
        self.0.clone()
    }
}

pub fn parse_furigana<'a>(
    input: &'a String,
    reading: &'a String,
    kanji_readings: &'a HashMap<char, Vec<String>>,
) -> Result<FuriganaString, Vec<chumsky::error::Simple<'a, char>>> {
    expression_parser(input, kanji_readings)
        .parse(reading)
        .into_result()
        .map(FuriganaString)
}

pub fn expression_parser<'a>(
    input: &'a String,
    kanji_readings: &'a HashMap<char, Vec<String>>,
) -> impl Parser<'a, &'a str, Vec<Furigana>, extra::Err<Simple<'a, char>>> + Clone {
    let mut parsers = vec![];

    for char in input.chars() {
        if wana_kana::utils::is_char_kanji(char) && kanji_readings.contains_key(&char) {
            let readings = kanji_readings.get(&char).unwrap();
            let kanji_parser = kanji_parser(char, readings.clone());
            parsers.push(kanji_parser.boxed());
        } else {
            let other_parser = just(char).map(|char: char| Furigana::Other(char));
            parsers.push(other_parser.boxed());
        }
    }

    let parser = series_parser(parsers);

    parser.boxed()
}

pub fn kanji_parser<'a>(
    kanji: char,
    mappings: Vec<String>,
) -> impl Parser<'a, &'a str, Furigana, extra::Err<Simple<'a, char>>> + Clone {
    let kanji = kanji.clone();
    choice(
        mappings
            .iter()
            .map(move |reading| {
                string_parser(reading.clone()).map(move |s: Vec<char>| {
                    let reading = s.into_iter().collect::<String>();
                    Furigana::Kanji {
                        character: kanji,
                        reading,
                    }
                })
            })
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_expression() {
        let k1 = '時';
        let k1_readings = vec!["とき", "じ", "どき"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let k2 = '間';
        let k2_readings = vec!["あいだ", "ま", "かん", "けん", "あい"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let word = "時間".to_string();
        let reading = "じかん".to_string();
        let kanji_readings = vec![(k1, k1_readings), (k2, k2_readings)]
            .into_iter()
            .collect::<HashMap<_, _>>();

        let result = parse_furigana(&word, &reading, &kanji_readings);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.to_writing(), word);
        assert_eq!(result.to_reading(), reading);
    }
}
