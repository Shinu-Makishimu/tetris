#![allow(dead_code)]
#![feature(array_chunks, slice_pattern)]

use engine::{Engine, Matrix, Color, piece::Kind as PieceKind};


mod engine;
mod interface;

fn main() {
    let mut matrix = Matrix::blank();
    for col in 0..=5{
        matrix[(col,0).into()] = Some(Color::Green);
    }
    for col in 0..=2{
        matrix[(col,1).into()] = Some(Color::Yellow);
    }

    let mut engine = Engine::with_matrix(matrix);

    engine.db_test_cursor(PieceKind::I, (5,5).into());
    
    interface::run(engine)
}
