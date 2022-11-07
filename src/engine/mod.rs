
use cgmath::Vector2;
use rand::{prelude::{SliceRandom, ThreadRng}, thread_rng};
use self::piece::{Piece, Kind as PieceKind};

mod piece;

type Coordinate = Vector2<usize>;
type Offset = Vector2<isize>;

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

    fn refill_bag(&mut self) {
        debug_assert!(self.bag.is_empty());
        self.bag.extend_from_slice(PieceKind::ALL.as_slice());
        self.bag.shuffle(&mut self.rng)
    }

    fn place_cursor(&mut self,) {
        let cursor = self.cursor.take().expect("Called place cursor without cursor");
        
        for coordinate in cursor.cells().expect("Corsor was out if bounds") {
            let cell: &mut bool = self.board.get_mut(coordinate).unwrap();
            debug_assert_eq!(*cell, false);
            *cell = true;

        }
    }
}

struct Board([bool;Self::SIZE]);

impl Board {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;
    const SIZE: usize = Self::HEIGHT * Self::WIDTH;

    fn in_bounds(Coordinate {x, y}: Coordinate) -> bool {
        x < Self::WIDTH && y < Self::HEIGHT
    }

    fn indexing(Coordinate {x, y}: Coordinate) -> usize{
        y * Self::WIDTH + x
    }

    fn blank() -> Self{
        Self([false; Self::SIZE])
    }

    fn get_mut(&mut self, coord: Coordinate) -> Option<&mut bool> {
        Self::in_bounds(coord).then(|| &mut self.0[Self::indexing(coord)])
    }
}