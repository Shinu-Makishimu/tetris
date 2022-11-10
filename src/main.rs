#![allow(dead_code)]


use engine::{Engine, Matrix, Color, piece::Kind as PieceKind};


mod engine;
mod interface;

fn main() {
    let mut matrix = Matrix::blank();

    matrix[(1,1).into()] = Some(Color::Green);

    let mut engine = Engine::with_matrix(matrix);

    engine.db_test_cursor(PieceKind::T, (5,5).into());
    
    interface::run(engine)
}
