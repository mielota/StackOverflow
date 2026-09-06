use std::{fmt::format, process::exit};

use crate::card::{Rank, Suit};
use raylib::{self, RaylibHandle, RaylibThread, core::texture::Texture2D};
use strum::{EnumCount, VariantNames};

struct CardTextures {
    cards: [[Texture2D; Rank::COUNT]; Suit::COUNT],
    back: Texture2D,
    empty: Texture2D,
}

struct UiTextures {
    padlock: Texture2D,
}

struct ChessTextures {
    white: Texture2D,
    black: Texture2D,
}

pub struct Textures {
    card_textures: CardTextures,
    ui_textures: UiTextures,
    chess_textures: ChessTextures,
}

fn load_texture(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Texture2D {
    match rl.load_texture(thread, path) {
        Ok(texture) => texture,
        Err(error) => {
            eprintln!("Error loading texture {}: {}", path, error);
            exit(exitcode::NOINPUT);
        }
    }
}

impl CardTextures {

    const PATH_RANKS: [&'static str; 13] = [
        "A", "02", "03", "04", "05", "06", "07", "08", "09", "10", "J", "Q", "K",
    ];
    const PATH_SUITS: [&'static str; 4] = ["hearts", "diamonds", "clubs", "spades"];

    fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> CardTextures {
        let cards: [[Texture2D; Rank::COUNT]; Suit::COUNT] = std::array::from_fn(|suit| {
            std::array::from_fn(|rank| {
                load_texture(
                    rl,
                    thread,
                    &format(format_args!(
                        "assets/cards/card_{}_{}.png",
                        Self::PATH_SUITS[suit],
                        Self::PATH_RANKS[rank],
                    )),
                )
            })
        });

        let back = load_texture(rl, thread, "assets/cards/card_back.png");
        let empty = load_texture(rl, thread, "assets/cards/card_empty.png");

        Self { cards, back, empty }
    }
}

impl UiTextures {
    fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> UiTextures {
        Self { padlock: load_texture(rl, thread, "assets/ui/padlock.png") }
    }
}

impl ChessTextures {
    fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> ChessTextures {
        Self {
            white: load_texture(rl, thread, "assets/pieces/WhitePieces.png"),
            black: load_texture(rl, thread, "assets/pieces/BlackPieces.png"),
        }
    }
}

impl Textures {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Textures {
        Self {
            card_textures: CardTextures::new(rl, &thread),
            ui_textures: UiTextures::new(rl, &thread),
            chess_textures: ChessTextures::new(rl, &thread),
        }
    }
}
