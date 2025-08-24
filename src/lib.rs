use chumsky::prelude::*;
use std::collections::HashMap;
use wana_kana;

mod word_parser;

use word_parser::word_parser;

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
    furigana_parser(input, kanji_readings)
        .parse(reading)
        .into_result()
        .map(|v| {
            let v = v
                .into_iter()
                .map(|(character, reading)| {
                    if wana_kana::utils::is_char_kanji(character) || character == '々' {
                        Furigana::Kanji { character, reading }
                    } else {
                        Furigana::Other(character)
                    }
                })
                .collect();
            FuriganaString(v)
        })
}

pub fn furigana_parser<'a>(
    input: &'a String,
    kanji_readings: &'a HashMap<char, Vec<String>>,
) -> impl Parser<'a, &'a str, Vec<(char, String)>, extra::Err<Simple<'a, char>>> + Clone {
    let mut pairs = vec![];

    for char in input.chars() {
        if (wana_kana::utils::is_char_kanji(char) || char == '々')
            && kanji_readings.contains_key(&char)
        {
            let readings = kanji_readings.get(&char).unwrap();
            pairs.push((char.clone(), readings.clone()));
        } else {
            pairs.push((char.clone(), vec![char.to_string()]));
        }
    }

    let parser = word_parser(pairs);

    parser.boxed()
}

//Failed to parse word: 関連, reading: かんれん: readings: {'関': ["せき", "ぜき", "かか", "わる", "からくり", "かんぬき", "かん"], '連': ["つら", "なる", "つら", "ねる", "つ", "れる", "づ", "れ", "れん", "っ"]}
#[cfg(test)]
mod test {
    use super::*;

    //Failed to parse word: 着々, reading: ちゃくちゃく: readings: {'着': ["き", "る", "き", "せる", "つ", "く", "つ", "ける", "ちゃく", "じゃく", "っ", "っ"], '々': ["ぎ", "き", "る", "る", "ぎ", "き", "ぜる", "せる", "づ", "つ", "ぐ", "く", "づ", "つ", "げる", "ける", "ぢゃく", "ちゃく", "じゃく", "じゃく", "っ", "っ", "っ", "っ"]}
    #[test]
    fn test_parse_expression() {
        let cases = vec![
            (
                "時間".to_string(),
                "じかん".to_string(),
                vec![
                    (
                        '時',
                        vec!["とき".to_string(), "じ".to_string(), "どき".to_string()],
                    ),
                    (
                        '間',
                        vec![
                            "あいだ".to_string(),
                            "ま".to_string(),
                            "かん".to_string(),
                            "けん".to_string(),
                            "あい".to_string(),
                        ],
                    ),
                ],
            ),
            (
                "噴煙".to_string(),
                "ふんえん".to_string(),
                vec![
                    (
                        '噴',
                        vec!["ふ".to_string(), "く".to_string(), "ふん".to_string()],
                    ),
                    (
                        '煙',
                        vec![
                            "けむ".to_string(),
                            "る".to_string(),
                            "けむり".to_string(),
                            "けむ".to_string(),
                            "い".to_string(),
                            "えん".to_string(),
                        ],
                    ),
                ],
            ),
            (
                "関連".to_string(),
                "かんれん".to_string(),
                vec![
                    (
                        '関',
                        vec![
                            "せき".to_string(),
                            "ぜき".to_string(),
                            "かか".to_string(),
                            "わる".to_string(),
                            "からくり".to_string(),
                            "かんぬき".to_string(),
                            "かん".to_string(),
                        ],
                    ),
                    (
                        '連',
                        vec![
                            "つら".to_string(),
                            "なる".to_string(),
                            "つら".to_string(),
                            "ねる".to_string(),
                            "つ".to_string(),
                            "れる".to_string(),
                            "づ".to_string(),
                            "れ".to_string(),
                            "れん".to_string(),
                            "っ".to_string(),
                        ],
                    ),
                ],
            ),
            (
                "着々".to_string(),
                "ちゃくちゃく".to_string(),
                vec![
                    (
                        '着',
                        vec![
                            "き".to_string(),
                            "る".to_string(),
                            "き".to_string(),
                            "せる".to_string(),
                            "つ".to_string(),
                            "く".to_string(),
                            "つ".to_string(),
                            "ける".to_string(),
                            "ちゃく".to_string(),
                            "じゃく".to_string(),
                            "っ".to_string(),
                            "っ".to_string(),
                        ],
                    ),
                    (
                        '々',
                        vec![
                            "ぎ".to_string(),
                            "き".to_string(),
                            "る".to_string(),
                            "る".to_string(),
                            "ぎ".to_string(),
                            "き".to_string(),
                            "ぜる".to_string(),
                            "せる".to_string(),
                            "づ".to_string(),
                            "つ".to_string(),
                            "ぐ".to_string(),
                            "く".to_string(),
                            "づ".to_string(),
                            "つ".to_string(),
                            "げる".to_string(),
                            "ける".to_string(),
                            "ぢゃく".to_string(),
                            "ちゃく".to_string(),
                            "じゃく".to_string(),
                            "じゃく".to_string(),
                            "っ".to_string(),
                            "っ".to_string(),
                            "っ".to_string(),
                            "っ".to_string(),
                        ],
                    ),
                ],
            ),
        ];

        for (word, reading, kanji_readings) in cases {
            let kanji_readings: HashMap<char, Vec<String>> = kanji_readings
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(|s| s.to_string()).collect()))
                .collect();
            let result = parse_furigana(&word, &reading, &kanji_readings);
            // println!("{:?}", result);
            assert!(result.is_ok());
            let result = result.unwrap();
            assert_eq!(result.to_writing(), word);
            assert_eq!(result.to_reading(), reading);
        }
    }
}
