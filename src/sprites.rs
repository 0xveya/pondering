use crate::duck::DuckKind;

pub struct Sprite {
    pub pixels: &'static [[u8; 12]],
}

const ADULT: [[u8; 12]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 3, 3],
    [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0],
    [4, 0, 4, 4, 4, 4, 4, 4, 1, 0, 0, 0],
    [0, 4, 4, 5, 5, 5, 4, 4, 4, 0, 0, 0],
    [0, 4, 4, 4, 5, 5, 5, 4, 4, 0, 0, 0],
    [0, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0],
    [0, 0, 0, 4, 4, 4, 4, 0, 0, 0, 0, 0],
];

static MALLARD: Sprite = Sprite { pixels: &ADULT };

static DUCKLING: Sprite = Sprite {
    pixels: &[
        [0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 2, 3, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
        [4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0],
        [0, 4, 5, 5, 4, 4, 0, 0, 0, 0, 0, 0],
        [0, 0, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0],
    ],
};

pub fn get(kind: DuckKind) -> &'static Sprite {
    match kind {
        DuckKind::Mallard | DuckKind::White => &MALLARD,
        DuckKind::Duckling => &DUCKLING,
    }
}
