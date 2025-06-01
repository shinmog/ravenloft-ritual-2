use crate::symbol::Symbol;

pub fn load() -> [[[Symbol; 3]; 4]; 5] {
    [
        // 1
        [
            [
                Symbol::TwoHorizontalLines,
                Symbol::TriangleWithCross,
                Symbol::Square,
            ],
            [
                Symbol::SquareWithVerticalLine,
                Symbol::UpArrow,
                Symbol::UWithCross,
            ],
            [
                Symbol::BWithTail,
                Symbol::CircleWithCross,
                Symbol::SquareWithHorizontalLine,
            ],
            [
                Symbol::CircleWithHorizontalLine,
                Symbol::CircleWithVerticalLine,
                Symbol::BWithTail,
            ],
        ],
        // 2
        [
            [
                Symbol::TriangleWithCross,
                Symbol::UpsideDownUWithCross,
                Symbol::TriangleWithCross,
            ],
            [
                Symbol::VWithForwardSlash,
                Symbol::CircleWithHorizontalLine,
                Symbol::X,
            ],
            [
                Symbol::UWithHorizontalLine,
                Symbol::Triangle,
                Symbol::CircleWithCross,
            ],
            [
                Symbol::CircleWithVerticalLine,
                Symbol::SquareWithHorizontalLine,
                Symbol::SquareWithHorizontalLine,
            ],
        ],
        // 3
        [
            [
                Symbol::SquareWithHorizontalLine,
                Symbol::U,
                Symbol::Triangle,
            ],
            [
                Symbol::X,
                Symbol::SquareWithHorizontalLine,
                Symbol::BWithTail,
            ],
            [Symbol::X, Symbol::X, Symbol::BWithTail],
            [
                Symbol::CircleWithCross,
                Symbol::TriangleWithCross,
                Symbol::ThreeHorizontalLines,
            ],
        ],
        // 4
        [
            [
                Symbol::TwoHorizontalLines,
                Symbol::BWithTail,
                Symbol::Circle,
            ],
            [Symbol::X, Symbol::VWithForwardSlash, Symbol::BWithTail],
            [
                Symbol::SquareWithVerticalLine,
                Symbol::SquareWithHorizontalLine,
                Symbol::SquareWithVerticalLine,
            ],
            [Symbol::Triangle, Symbol::Square, Symbol::X],
        ],
        // 5
        [
            [
                Symbol::SquareWithHorizontalLine,
                Symbol::VWithForwardSlash,
                Symbol::UWithHorizontalLine,
            ],
            [
                Symbol::X,
                Symbol::CircleWithHorizontalLine,
                Symbol::CircleWithCross,
            ],
            [
                Symbol::TwoHorizontalLines,
                Symbol::CircleWithHorizontalLine,
                Symbol::UpsideDownUWithCross,
            ],
            [
                Symbol::TriangleWithVerticalLine,
                Symbol::Triangle,
                Symbol::CircleWithHorizontalLine,
            ],
        ],
    ]
}
