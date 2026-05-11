use crate::game::{dictionnary::Dictionnary, tile::TileState};

pub struct Validator {
    secret_word: String,
}

#[derive(Debug, PartialEq)]
pub enum SubmissionError {
    TooShort,
    NotInDictionnary,
}

impl Validator {
    pub fn validate(&self, submitted_word: &str) -> Result<Vec<TileState>, SubmissionError> {
        if submitted_word.len() < 5 {
            return Err(SubmissionError::TooShort);
        }

        if !Dictionnary::contains(&submitted_word) {
            return Err(SubmissionError::NotInDictionnary);
        }

        let secret: Vec<char> = self.secret_word.chars().collect();
        let guess: Vec<char> = submitted_word.chars().collect();
        let mut result = vec![TileState::Absent; 5];
        let mut secret_used = [false; 5];

        // Passe 1 : Correct
        for i in 0..5 {
            if guess[i] == secret[i] {
                result[i] = TileState::Correct;
                secret_used[i] = true;
            }
        }

        // Passe 2 : Present
        for i in 0..5 {
            if result[i] == TileState::Correct {
                continue;
            }
            for j in 0..5 {
                if !secret_used[j] && guess[i] == secret[j] {
                    result[i] = TileState::Present;
                    secret_used[j] = true;
                    break;
                }
            }
        }

        Ok(result)
    }

    pub fn new(secret_word: String) -> Self {
        Self {
            secret_word: secret_word.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{tile::TileState, validator::SubmissionError};

    use super::Validator;

    #[test]
    fn test_submission_too_short() {
        let v = Validator::new("POMME".to_owned());
        let result = v.validate("LONG");

        assert!(result.is_err());
        assert_eq!(Err(SubmissionError::TooShort), result);
    }

    #[test]
    fn test_validate() {
        let v = Validator {
            secret_word: "POILS".to_owned(),
        };

        let returned = v.validate("PALME");
        if let Ok(r) = returned {
            assert_eq!(r[0], TileState::Correct);
            assert_eq!(r[1], TileState::Absent);
            assert_eq!(r[2], TileState::Present);
            assert_eq!(r[3], TileState::Absent);
            assert_eq!(r[4], TileState::Absent);
        }

        let returned = v.validate("POIRE");
        if let Ok(r) = returned {
            assert_eq!(r[0], TileState::Correct);
            assert_eq!(r[1], TileState::Correct);
            assert_eq!(r[2], TileState::Correct);
            assert_eq!(r[3], TileState::Absent);
            assert_eq!(r[4], TileState::Absent);
        }

        let returned = v.validate("POILS");
        if let Ok(r) = returned {
            assert_eq!(r[0], TileState::Correct);
            assert_eq!(r[1], TileState::Correct);
            assert_eq!(r[2], TileState::Correct);
            assert_eq!(r[3], TileState::Correct);
            assert_eq!(r[4], TileState::Correct);
        }
    }

    #[test]
    fn test_double_letter_correct_steals_present() {
        // Secret POMME a 2 M. L'essai MMMAX a 3 M dont un Correct en pos 2.
        // Le Correct consomme un M, il reste un M libre → pos 0 = Present.
        // Mais pos 1 n'a plus de M disponible → Absent (pas Present).
        let v = Validator::new("POMME".to_owned());
        let returned = v.validate("MMMAX");
        if let Ok(r) = returned {
            assert_eq!(r[0], TileState::Present);
            assert_eq!(r[1], TileState::Absent);
            assert_eq!(r[2], TileState::Correct);
            assert_eq!(r[3], TileState::Absent);
            assert_eq!(r[4], TileState::Absent);
        }
    }

    #[test]
    fn test_validate_multiple_times_same_letter() {
        let v = Validator::new("POMME".to_owned());

        let returned = v.validate("PALME");
        if let Ok(r) = returned {
            assert_eq!(r[0], TileState::Correct);
            assert_eq!(r[1], TileState::Absent);
            assert_eq!(r[2], TileState::Absent);
            assert_eq!(r[3], TileState::Correct);
            assert_eq!(r[4], TileState::Correct);
        }

        let returned = v.validate("POMME");
        if let Ok(r) = returned {
            assert_eq!(r[0], TileState::Correct);
            assert_eq!(r[1], TileState::Correct);
            assert_eq!(r[2], TileState::Correct);
            assert_eq!(r[3], TileState::Correct);
            assert_eq!(r[4], TileState::Correct);
        }
    }
}
