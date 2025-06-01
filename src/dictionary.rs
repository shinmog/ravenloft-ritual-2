use crate::pillar;
use crate::state::State;
use crate::symbol::Symbol;

#[derive(Debug)]
pub struct Dictionary {
    pub translations: Vec<(Symbol, State, u8)>,
}

impl Dictionary {
    pub fn lookup(&self, symbol: &Symbol) -> usize {
        for idx in 0..self.translations.len() {
            if &self.translations[idx].0 == symbol {
                return idx;
            }
        }

        unreachable!()
    }

    pub fn load() -> Self {
        // NOTE: Update me as we learn more.
        let mut xlate: Vec<(Symbol, State, u8)> = vec![
            (Symbol::CircleWithVerticalLine, State::Known('H'), 0),
            (Symbol::CircleWithHorizontalLine, State::Known('N'), 0),
            (Symbol::BWithTail, State::Known('E'), 0),
            (Symbol::X, State::Known('O'), 0),
            (Symbol::CircleWithCross, State::Known('T'), 0),
            (Symbol::SquareWithHorizontalLine, State::Known('R'), 0),
            (Symbol::TriangleWithCross, State::Known('W'), 0),
            (Symbol::VWithForwardSlash, State::Known('A'), 0),
            (Symbol::U, State::Guess('G'), 0),
            (Symbol::Triangle, State::Guess('D'), 0),
            (Symbol::ThreeHorizontalLines, State::Guess('P'), 0),
        ];

        // All unmentioned symbols are unknowns.
        for group in pillar::load().iter() {
            for row in group.iter() {
                for column in row.iter() {
                    let mut found: bool = false;
                    for (symbol, _, count) in xlate.iter_mut() {
                        if column == symbol {
                            found = true;
                            *count += 1;
                            break;
                        }
                    }
                    if !found {
                        xlate.push((column.clone(), State::Unknown, 1));
                    }
                }
            }
        }

        // Sort: desc.
        xlate.sort_by(|(_, _, a), (_, _, b)| b.cmp(&a));

        Self {
            translations: xlate,
        }
    }
}
