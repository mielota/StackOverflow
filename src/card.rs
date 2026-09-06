use bitflags::bitflags;
use std::cmp::{max, min};
use strum_macros::{EnumCount, EnumIter, EnumString, VariantNames};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter, EnumString, VariantNames)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, EnumIter, EnumString, VariantNames)]
pub enum Rank {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn value(self) -> u64 {
        self as u64
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct State: u64 {
        const NONE = 0;
        const LOCKED = 1 << 1;
        const GLITCHED = 1 << 2;
        const ROTTED = 1 << 3;
        const HIDDEN = 1 << 4;
        const EPHEMERAL = 1 << 5;
        const DISCOUNTED = 1 << 6;
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub state: State,
    pub rot_turns: u64,
}

impl Card {
    const CARD_ROT_CAP_TURNS: u64 = 3;

    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            suit,
            rank,
            state: State::NONE,
            rot_turns: 0,
        }
    }

    pub fn effective_value(&self) -> u64 {
        self.rank.value() * self.rot_multiplier()
    }

    pub fn chip_value(&self) -> u64 {
        if self.rank == Rank::Ace {
            11
        } else {
            self.rank.value()
        }
    }

    pub fn rot_multiplier(&self) -> u64 {
        if !self.is(State::ROTTED) {
            1
        } else {
            1 << max(1, min(self.rot_turns, Card::CARD_ROT_CAP_TURNS))
        }
    }

    pub fn rot(&mut self) {
        if !self.is(State::ROTTED) {
            self.mark(State::GLITCHED, false);
            self.mark(State::ROTTED, true);
            self.rot_turns = 1;
        }
    }

    pub fn age_rot(&mut self) {
        if !self.is(State::ROTTED) {
            if self.rot_turns < Card::CARD_ROT_CAP_TURNS {
                self.rot_turns += 1;
            }
        }
    }

    pub fn glitch(&mut self) {
        if !self.is(State::GLITCHED) {
            self.mark(State::ROTTED, false);
            self.mark(State::GLITCHED, true);
            self.rot_turns = 0;
        }
    }

    pub fn clear_rot_and_glitch(&mut self) {
        self.mark(State::GLITCHED, false);
        self.mark(State::ROTTED, false);
        self.rot_turns = 0;
    }

    pub fn is(&self, state: State) -> bool {
        (self.state & state) != State::NONE
    }

    pub fn mark(&mut self, state: State, value: bool) {
        if value {
            self.state |= state;
        } else {
            self.state &= !state;
        }
    }
}
