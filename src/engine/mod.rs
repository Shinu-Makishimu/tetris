
use std::ops::{Index, IndexMut};

use cgmath::{Vector2, Point2};
use rand::{prelude::{SliceRandom, ThreadRng}, thread_rng};
use self::piece::{Piece, Kind as PieceKind};

mod piece;

type Coordinate = Point2<usize>;
type Offset = Vector2<isize>;

#[derive(Copy,Clone, PartialEq, Debug)]
pub enum MoveKind { Left, Right }


impl MoveKind {
    fn offset(&self) -> Offset {
        match self {
            MoveKind::Left => Offset::new(-1,0),
            MoveKind::Right => Offset::new(1, 0),
        }
    }
}

pub struct Engine {
    matrix: Matrix,
    bag: Vec<PieceKind>,
    rng: ThreadRng,
    cursor: Option<Piece>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            matrix: Matrix::blank(),
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
        
        assert!(
            self.matrix.placeable(&cursor), 
            "Trying to place cursor in unplaceble loc {:?}", cursor
        );
        
        let color = cursor.kind.color();
        for coordinate in cursor.cells().unwrap() {
            self.matrix[coordinate]  = Some(color);

        }
    }

    fn move_cursor(&mut self, move_kind: MoveKind) -> Result<(), ()> {  //ok or err
        let Some(cursor) = self.cursor.as_mut() else {  //new feature in naitly release
            return Ok(());
        };

        let new = cursor.moved_by(move_kind.offset());

        if self.matrix.is_clipping(&new) {
            return Err(());
            
        }
        Ok(self.cursor = Some(new))

    }

    fn step_down(&mut self ) {
        self.cursor = Some(self.ticked_down_cursor().unwrap());

    }

    pub fn cursor_hit_down(&self) -> bool {
        self.cursor.is_some() &&
        self.ticked_down_cursor().is_none()

    }

    fn ticked_down_cursor(&self) -> Option<Piece> {
        let Some(cursor) = self.cursor else {
            return None; 
        };
        let new = cursor.moved_by(Offset::new(0,-1));
        (!self.matrix.is_clipping(&new)).then_some(new)
    }

    fn hard_drop(&mut self) {
        while let Some(new) = self.ticked_down_cursor() {
            self.cursor = Some(new);
        }
        self.place_cursor();        
    }



}


#[derive(Copy,Clone, PartialEq, Debug)]
pub enum Color {Yellow, Cyan, Purple, Orange, Blue, Green, Red}




struct Matrix([Option<Color>;Self::SIZE]);

impl Matrix {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 20;
    const SIZE: usize = Self::HEIGHT * Self::WIDTH;

    fn on_matrix(coord: Coordinate) -> bool {
        //x < Self::WIDTH && y < Self::HEIGHT
        Self::valid_coord(coord) && coord.y < Self::HEIGHT
    }

    fn valid_coord(coord: Coordinate) -> bool {
        coord.x < Self::WIDTH 
    }

    fn indexing(Coordinate {x, y}: Coordinate) -> usize{
        y * Self::WIDTH + x
    }

    fn blank() -> Self{
        Self([None; Self::SIZE])
    }


    fn is_clipping(&self, piece: &Piece) -> bool {
        let Some(cells) = piece.cells() else {
            return true;
        };
        cells.into_iter().any(|coord|
            !Matrix::on_matrix(coord) || self[coord].is_some()
        )
    }


    fn placeable(&self, piece: &Piece) -> bool {
        let Some(cells) = piece.cells() else {
            return false;
        };
        cells.into_iter().all(|coord|
            Matrix::on_matrix(coord) && self[coord].is_none()
        )

    }


}

impl Index<Coordinate> for Matrix {
    type Output = Option<Color>;

    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(Self::on_matrix(coord));
        &self.0[Self::indexing(coord)]
    }
}

impl IndexMut<Coordinate> for Matrix {
    
    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        assert!(Self::on_matrix(coord));
        &mut self.0[Self::indexing(coord)]
    }
}