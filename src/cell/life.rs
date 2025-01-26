use std::str::FromStr;
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::cell::Cell;

#[derive(PartialEq, Eq)]
pub enum Life {
    Dead,
    Alive,
}

#[derive(Clone, Debug)]
pub struct LifeRule {
    birth: u16,
    survive: u16,
}

#[derive(Debug)]
pub struct RuleParseError {
    pub message: &'static str,
    pub reason: RuleParseReason,
}

#[derive(Debug)]
pub enum RuleParseReason {
    Format,
    DigitParse,
}

impl LifeRule {
    fn from_digits(mut chars: impl Iterator<Item = char>) -> Result<u16, RuleParseError> {
        let mut rule: u16 = 0;
        while let Some(char) = chars.next() {
            match char.to_digit(10) {
                Some(digit) => {
                    if digit > 8 {
                        return Err(RuleParseError {
                            message: "digit can only be 0 through 8",
                            reason: RuleParseReason::DigitParse
                        });
                    }

                    // otherwise, update the rule
                    rule |= 1 << digit;
                },
                None => return Err(RuleParseError {
                    message: "rule can only include digits",
                    reason: RuleParseReason::DigitParse
                })
            }
        }

        Ok(rule)
    }
}

impl FromStr for LifeRule {
    type Err = RuleParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();

        // check that the string starts with a 'B'
        if chars.next() != Some('B') {
            return Err(RuleParseError {
                message: "life rule must start with a birth rule",
                reason: RuleParseReason::Format
            });
        }

        // find the index of the 'S'
        let survive_start = s.find('S').ok_or_else(|| RuleParseError {
            message: "life rule must include a survival rule",
            reason: RuleParseReason::Format
        })?;

        let birth_chars = s.chars().take(survive_start).skip(1);
        let survive_chars = s.chars().skip(survive_start + 1);
        Ok(LifeRule {
            birth: LifeRule::from_digits(birth_chars)?,
            survive: LifeRule::from_digits(survive_chars)?
        })
    }
}

pub struct LifeParams {
    pub alive_ratio: f32,
    pub rule: LifeRule,
}

impl Cell for Life {
    type Params = LifeParams;

    fn new(params: &Self::Params) -> Self {
        if gen_range(0.0, 1.0) < params.alive_ratio {
            Life::Alive
        }
        else {
            Life::Dead
        }
    }

    fn next<'a>(&'a self, params: &Self::Params, neighbors: impl IntoIterator<Item = &'a Self>) -> Self
    {
        let count = neighbors.into_iter()
            .filter(|neighbor| **neighbor == Life::Alive)
            .count();

        match *self {
            Life::Dead => match params.rule.birth & (1 << count) > 0 {
                true => Life::Alive,
                false => Life::Dead
            },
            Life::Alive => match params.rule.survive & (1 << count) > 0 {
                true => Life::Alive,
                false => Life::Dead,
            },
        }
    }

    fn color(&self, _params: &Self::Params) -> Color {
        match *self {
            Life::Dead => WHITE,
            Life::Alive => BLACK,
        }
    }
}

impl Default for Life {
    fn default() -> Self {
        Life::Dead
    }
}
