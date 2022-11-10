use std::{ops::{Index, IndexMut}, time::Duration};
use cgmath::{Vector2, Point2, EuclideanSpace};
use rand::{prelude::{SliceRandom, ThreadRng}, thread_rng};
use self::{piece::{Piece, Kind as PieceKind, Rotation},geometry::GridIncrement} ;

pub mod piece;
mod geometry;

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
    level: u8,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            matrix: Matrix::blank(),
            bag: Vec::new(),
            rng: thread_rng(),
            cursor: None,
            level: 1,
        }
    }

    pub fn with_matrix(matrix:Matrix) ->Self {
        Self {
            matrix,
            ..Self::new()
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

    pub fn move_cursor(&mut self, move_kind: MoveKind) -> Result<(), ()> {  //ok or err
        let Some(cursor) = self.cursor.as_mut() else {  //new feature in naitly release
            return Ok(());
        };

        let new = cursor.moved_by(move_kind.offset());

        if self.matrix.is_clipping(&new) {
            return Err(());
            
        }
        self.cursor = Some(new);
        Ok(())
    }

    pub fn cursor_info(&self) -> Option<([Coordinate; Piece::CELL_COUNT], Color)> {
        let cursor = self.cursor?;
        Some((cursor.cells().unwrap(), cursor.kind.color()))
    }

    pub fn db_test_cursor(&mut self, kind: PieceKind, position: Offset) {
        let piece = Piece {kind, rotation: Rotation::N, position};
        self.cursor = Some(piece);
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

    pub fn hard_drop(&mut self) {
        while let Some(new) = self.ticked_down_cursor() {
            self.cursor = Some(new);
        }
        self.place_cursor();        
    }

    pub fn cells(&self) ->CellIter <'_> {
        CellIter {
            position: Coordinate::origin(),
            cell_iter: self.matrix.0.iter(),
        }
    }

    pub fn drop_time(&self) -> Duration {
        let level_index = self.level - 1;
        let sec_per_line  = (0.8 - (level_index as f32 * 0.007 )).powi(level_index as _);
        Duration::from_secs_f32(sec_per_line)
    }
}


#[derive(Copy,Clone, PartialEq, Debug)]
pub enum Color {Yellow, Cyan, Purple, Orange, Blue, Green, Red}

pub struct Matrix([Option<Color>; Self::SIZE]);


impl Matrix {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 20;
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

    pub fn blank() -> Self{
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


pub struct CellIter<'matrix> {
    position: Coordinate,
    cell_iter: ::std::slice::Iter<'matrix, Option<Color>>, //here <'matrix> is lifetime parameter
}

impl<'matrix> Iterator for CellIter<'matrix>{
    type Item = (Coordinate, Option<Color>) ;

    fn next(&mut self) -> Option<Self::Item> {
        /*if let Some(cell) = self.cells.next() {
            let coord = self.position;

            return Some((coord, cell));
        } else {
            None
        }*/ //this piece of code is convert into let-some-else

        let Some(&cell) = self.cell_iter.next() else {
            return None;
        };
        let coord = self.position;

        /*self.position.x += 1;
        self.position.x %= Matrix::WIDTH;
        if self.position.x == 0 {
            self.position.y += 1;
        }*/ // moved to geometry

        self.position.grid_inc();
        Some((coord, cell))
    } 
}


#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn cell_iter() {
        let mut matrix = Matrix::blank();
        matrix[Coordinate::new(2,0)] = Some(Color::Blue);
        matrix[Coordinate::new(3,1)] = Some(Color::Green);

        let mut iter = CellIter {
            position: Coordinate::origin(),
            cell_iter: matrix.0.iter(),
        };

        let first_five = (&mut iter).take(5).collect::<Vec<_>>();
        
        assert_eq!(first_five, vec![
            (Coordinate::new(0,0), None),
            (Coordinate::new(1,0), None),
            (Coordinate::new(2,0), Some(Color::Blue)),
            (Coordinate::new(3,0), None),
            (Coordinate::new(4,0), None),
        ]);

        let other_item = (&mut iter).skip(8).next();
        assert_eq!(
            other_item, 
            Some((Coordinate::new(3,1), Some(Color::Green)))
        );


        assert!(iter.all(|(_, contents)| contents.is_none()));


    }
}