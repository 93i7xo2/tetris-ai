use common::model::{Bag, Game, GameMove};

fn main() {
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
    let mut game = Game::new(&bag);
    game.make_move(GameMove::Hold);
    game.allow_hold();
    for _ in 0..10_000 {
        for game_move in moves.iter() {
            game.make_move(*game_move);
            game.refill_queue(&bag);
            println!("{}", game);
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}
