use crate::game::tile::TileState;

pub struct Validator {
    secret_word: String,
}

pub enum SubmissionError {
    TooShort,
    NotInDictionnary,
}

impl Validator {
    pub fn validate(&self, submitted_word: String) -> Result<Vec<TileState>, SubmissionError> {
        if submitted_word.len() < 5 {
            return Err(SubmissionError::TooShort);
        }

        // TODO validate against dictionnary

        let chars = submitted_word.chars();
        let mut result = Vec::new();

        for (i, char) in chars.enumerate() {
            let mut to_push: Option<TileState> = None;

            if self.secret_word.chars().collect::<Vec<_>>()[i] == char {
                to_push = Some(TileState::Correct);
            } else {
                let sw_chars = self.secret_word.chars();

                for (j, swc) in sw_chars.enumerate() {
                    if char == swc {
                        if i == j {
                            to_push = Some(TileState::Correct);
                        } else {
                            to_push = Some(TileState::Present);
                        }
                        break;
                    }
                }
            }

            if let Some(p) = to_push {
                result.push(p);
            } else {
                result.push(TileState::Absent);
            }
        }

        Ok(result)
    }

    pub fn new(arg: &str) -> Self {
        Self { secret_word: arg.to_owned() }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::tile::TileState;

    use super::Validator;

    #[test]
    fn test_validate() {
        let v = Validator {
            secret_word: "POILS".to_owned(),
        };

        let returned = v.validate("PALME".to_owned());
        if let Ok(r) = returned {
            assert_eq!(r[0], TileState::Correct);
            assert_eq!(r[1], TileState::Absent);
            assert_eq!(r[2], TileState::Present);
            assert_eq!(r[3], TileState::Absent);
            assert_eq!(r[4], TileState::Absent);
        }

        let returned = v.validate("POIRE".to_owned());
        if let Ok(r) = returned {
            assert_eq!(r[0], TileState::Correct);
            assert_eq!(r[1], TileState::Correct);
            assert_eq!(r[2], TileState::Correct);
            assert_eq!(r[3], TileState::Absent);
            assert_eq!(r[4], TileState::Absent);
        }

        let returned = v.validate("POILS".to_owned());
        if let Ok(r) = returned {
            assert_eq!(r[0], TileState::Correct);
            assert_eq!(r[1], TileState::Correct);
            assert_eq!(r[2], TileState::Correct);
            assert_eq!(r[3], TileState::Correct);
            assert_eq!(r[4], TileState::Correct);
        }
    }

    #[test]
    fn test_validate_multiple_times_same_letter() {
        let v = Validator::new("POMME");

        let returned = v.validate("PALME".to_owned());
        if let Ok(r) = returned {
            assert_eq!(r[0], TileState::Correct);
            assert_eq!(r[1], TileState::Absent);
            assert_eq!(r[2], TileState::Absent);
            assert_eq!(r[3], TileState::Correct);
            assert_eq!(r[4], TileState::Correct);
        }

        let returned = v.validate("POMME".to_owned());
        if let Ok(r) = returned {
            assert_eq!(r[0], TileState::Correct);
            assert_eq!(r[1], TileState::Correct);
            assert_eq!(r[2], TileState::Correct);
            assert_eq!(r[3], TileState::Correct);
            assert_eq!(r[4], TileState::Correct);
        }
    }
}
