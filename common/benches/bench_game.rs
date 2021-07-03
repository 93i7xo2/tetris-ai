#![feature(test)]
extern crate test;
use common::model::Bag;
use common::model::Game;
use common::model::GameMove;
use test::{black_box, Bencher};

#[bench]
fn dt_cannon_loop(b: &mut Bencher) {
    // moves is 175 GameMoves long
    // Assumption: Queue starts with [O] I T L ...
    let moves = vec![
        GameMove::Hold,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::HardDrop,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::HardDrop,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateRight,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateLeft,
        GameMove::RotateLeft,
        GameMove::SoftDrop,
        GameMove::RotateLeft,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateLeft,
        GameMove::RotateLeft,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateLeft,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::SoftDrop,
        GameMove::RotateRight,
        GameMove::HardDrop,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateLeft,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::RotateLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateLeft,
        GameMove::HardDrop,
        GameMove::RotateLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
        GameMove::RotateLeft,
        GameMove::ShiftRight,
        GameMove::SoftDrop,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::ShiftRight,
        GameMove::HardDrop,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::HardDrop,
        GameMove::RotateRight,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::SoftDrop,
        GameMove::RotateLeft,
        GameMove::HardDrop,
        GameMove::Hold,
        GameMove::Rotate180,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::ShiftLeft,
        GameMove::HardDrop,
    ];
    let bag = Bag::new_7_bag();
    b.iter(|| {
        let mut game = Game::new(&bag);
        game.make_move(GameMove::Hold);
        game.allow_hold();
        for game_move in moves.iter() {
            if game.queue_pieces.len() == 0 {
                game.extend_queue(&bag)
            }
            game.make_move(*game_move);
        }
        black_box(game);
    })
}

#[bench]
fn copy_game(b: &mut Bencher) {
    let bag = Bag::new_7_bag();
    let game = Game::new(&bag);
    b.iter(|| {
        let copy = game.clone();
        black_box(copy);
    })
}
