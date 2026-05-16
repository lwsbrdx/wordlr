use std::{collections::HashSet, hash::Hasher, sync::OnceLock};

use chrono::NaiveDate;
use seeded_random::{Random, Seed};

static WORDS: OnceLock<Vec<String>> = OnceLock::new();
static WORDS_SET: OnceLock<HashSet<String>> = OnceLock::new();

fn words() -> &'static Vec<String> {
    WORDS.get_or_init(|| {
        serde_json::from_str(include_str!("../dictionnary.json"))
            .expect("dictionnary.json invalide")
    })
}

fn words_set() -> &'static HashSet<String> {
    WORDS_SET.get_or_init(|| {
        HashSet::from_iter(words().clone())
    })
}

#[derive(Debug)]
pub(crate) struct Dictionnary {
    available_words: &'static Vec<String>,
}

impl Dictionnary {
    pub fn new() -> Self {
        Self {
            available_words: words(),
        }
    }

    fn date_to_hash(&self, date: NaiveDate) -> u64 {
        let formatted_date = date.format("%Y-%-m-%-d").to_string();
        let mut hasher = std::hash::DefaultHasher::new();
        hasher.write(formatted_date.as_bytes());
        hasher.finish()
    }

    pub(crate) fn get_word_for_day(&self, date: NaiveDate) -> String {
        let seed = Seed::unsafe_new(self.date_to_hash(date));
        let rng = Random::from_seed(seed);

        let random_ratio = rng.u32() as f32 / u32::MAX as f32;
        let index = match self.available_words.iter().position(|w| w == "NAVAL") {
            Some(index) => index + 1,
            None => self.available_words.len() - 1,
        };
        let random_index = (random_ratio * index as f32).round() as usize;

        self.available_words[random_index].clone()
    }

    pub(crate) fn contains(word: &str) -> bool {
        words_set().contains(word)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::NaiveDate;
    use std::str::FromStr;

    #[test]
    fn test_get_word_for_day() {
        let d = Dictionnary::new();
        let w = d.get_word_for_day(NaiveDate::from_str("2026-01-02").unwrap());
        assert_eq!("OUAIS", w);
    }
}
