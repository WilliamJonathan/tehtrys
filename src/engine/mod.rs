use rand::{prelude::{SliceRandom, ThreadRng}, thread_rng};
use self::piece::{Piece, Kind as PieceKind};

mod piece;

type Coordinate = cgmath::Vector2<usize>;
type Offset = cgmath::Vector2<isize>;

pub struct Engine {
    board: Board,
    bag: Vec<PieceKind>,
    rng: ThreadRng,
    cursor: Option<Piece>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::blank(),
            bag: Vec::new(),
            rng: thread_rng(),
            cursor: None,
        }
    }

    fn refil_bag(&mut self) {
        debug_assert!(self.bag.is_empty());
        self.bag.extend_from_slice(PieceKind::ALL.as_slice());
        self.bag.shuffle(&mut self.rng);
    }

    fn place_cursor(&mut self) {
        // Afirma que a peça não se sobrepõe às células preenchidas
        
    }
}

struct Board([bool;Self::SIZE]);

impl Board {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    fn in_bounds(Coordinate { x, y }: Coordinate) -> bool  {
        x < Self::WIDTH && y < Self::HEIGHT
    }

    fn blank() -> Self {
        Self([false; Self::SIZE])
    }

}