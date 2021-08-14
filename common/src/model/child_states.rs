use super::game::{Game, GameMove};
use std::collections::{hash_map::Entry, HashMap};
use std::lazy::SyncLazy;

/// Represents a child state of a game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChildState<'a> {
    pub game: Game,
    pub moves: &'a [GameMove],
}

impl Game {
    // Given a list of list of moves: &[Vec<GameMove>]
    // Return an array of unique child states
    // Which includes a game state plus a list of moves used to get there
    pub fn child_states<'a>(&self, moves_list: &'a [Vec<GameMove>]) -> Vec<ChildState<'a>> {
        let mut child_states = Vec::<ChildState<'a>>::new();
        let mut map = HashMap::<Game, usize>::new();
        for moves in moves_list {
            let mut game = self.clone();
            for game_move in moves {
                game.make_move(*game_move);
            }
            // Ignore topped-out games
            if game.board.topped_out() {
                continue;
            }
            match map.entry(game) {
                Entry::Occupied(entry) => {
                    let index = entry.get();
                    let other_moves = child_states[*index].moves;
                    if moves.len() < other_moves.len() {
                        // Replace with faster moves
                        child_states[*index].moves = moves;
                    }
                }
                Entry::Vacant(entry) => {
                    child_states.push(ChildState { game, moves });
                    entry.insert(child_states.len() - 1);
                }
            }
        }
        child_states
    }
}

static FRAGMENT_HOLD: SyncLazy<Vec<Vec<GameMove>>> =
    SyncLazy::new(|| vec![vec![], vec![GameMove::Hold]]);
static FRAGMENT_ROT: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    vec![
        vec![],
        vec![GameMove::RotateRight],
        vec![GameMove::Rotate180],
        vec![GameMove::RotateLeft],
    ]
});
static FRAGMENT_SHIFT: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    vec![
        vec![GameMove::ShiftLeft; 5],
        vec![GameMove::ShiftLeft; 4],
        vec![GameMove::ShiftLeft; 3],
        vec![GameMove::ShiftLeft; 2],
        vec![GameMove::ShiftLeft; 1],
        vec![],
        vec![GameMove::ShiftRight; 1],
        vec![GameMove::ShiftRight; 2],
        vec![GameMove::ShiftRight; 3],
        vec![GameMove::ShiftRight; 4],
        vec![GameMove::ShiftRight; 5],
    ]
});
static FRAGMENT_FINAL: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    vec![
        vec![],
        vec![GameMove::ShiftLeft],
        vec![GameMove::ShiftRight],
        vec![GameMove::RotateLeft],
        vec![GameMove::Rotate180],
        vec![GameMove::RotateRight],
    ]
});

pub static MOVES_0F: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let mut moves_list = Vec::new();
    for hold in &*FRAGMENT_HOLD {
        for rot in &*FRAGMENT_ROT {
            for shift in &*FRAGMENT_SHIFT {
                let mut moves = Vec::new();
                moves.extend(hold);
                moves.extend(rot);
                moves.extend(shift);
                moves.push(GameMove::HardDrop);
                moves_list.push(moves);
            }
        }
    }
    moves_list
});
pub static MOVES_1F: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let mut moves_list = Vec::new();
    for hold in &*FRAGMENT_HOLD {
        for rot in &*FRAGMENT_ROT {
            for shift in &*FRAGMENT_SHIFT {
                for final_1 in &*FRAGMENT_FINAL {
                    let f1 = final_1.len() > 0;
                    let mut moves = Vec::new();
                    moves.extend(hold);
                    moves.extend(rot);
                    moves.extend(shift);
                    if f1 {
                        moves.push(GameMove::SoftDrop);
                    }
                    moves.extend(final_1);
                    moves.push(GameMove::HardDrop);
                    moves_list.push(moves);
                }
            }
        }
    }
    moves_list
});

pub static MOVES_2F: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let mut moves_list = Vec::new();
    for final_1 in &*FRAGMENT_FINAL {
        for final_2 in &*FRAGMENT_FINAL {
            for hold in &*FRAGMENT_HOLD {
                for rot in &*FRAGMENT_ROT {
                    for shift in &*FRAGMENT_SHIFT {
                        let f1 = final_1.len() > 0;
                        let f2 = final_2.len() > 0;
                        if !f1 && f2 {
                            continue;
                        }
                        let mut moves = Vec::new();
                        moves.extend(hold);
                        moves.extend(rot);
                        moves.extend(shift);
                        if f1 {
                            moves.push(GameMove::SoftDrop);
                        }
                        moves.extend(final_1);
                        if f2 {
                            moves.push(GameMove::SoftDrop);
                        }
                        moves.extend(final_2);
                        moves.push(GameMove::HardDrop);
                        moves_list.push(moves);
                    }
                }
            }
        }
    }
    moves_list
});

pub static MOVES_3F: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let mut moves_list = Vec::new();
    for final_1 in &*FRAGMENT_FINAL {
        for final_2 in &*FRAGMENT_FINAL {
            for final_3 in &*FRAGMENT_FINAL {
                for hold in &*FRAGMENT_HOLD {
                    for rot in &*FRAGMENT_ROT {
                        for shift in &*FRAGMENT_SHIFT {
                            let f1 = final_1.len() > 0;
                            let f2 = final_2.len() > 0;
                            let f3 = final_3.len() > 0;
                            if (!f1 && f2) || (!f1 && !f2 && f3) {
                                continue;
                            }
                            let mut moves = Vec::new();
                            moves.extend(hold);
                            moves.extend(rot);
                            moves.extend(shift);
                            if f1 {
                                moves.push(GameMove::SoftDrop);
                            }
                            moves.extend(final_1);
                            if f2 {
                                moves.push(GameMove::SoftDrop);
                            }
                            moves.extend(final_2);
                            if f3 {
                                moves.push(GameMove::SoftDrop);
                            }
                            moves.extend(final_3);
                            moves.push(GameMove::HardDrop);
                            moves_list.push(moves);
                        }
                    }
                }
            }
        }
    }
    moves_list
});

pub static MOVES_4F: SyncLazy<Vec<Vec<GameMove>>> = SyncLazy::new(|| {
    let mut moves_list = Vec::new();
    for final_1 in &*FRAGMENT_FINAL {
        for final_2 in &*FRAGMENT_FINAL {
            for final_3 in &*FRAGMENT_FINAL {
                for final_4 in &*FRAGMENT_FINAL {
                    for hold in &*FRAGMENT_HOLD {
                        for rot in &*FRAGMENT_ROT {
                            for shift in &*FRAGMENT_SHIFT {
                                let f1 = final_1.len() > 0;
                                let f2 = final_2.len() > 0;
                                let f3 = final_3.len() > 0;
                                let f4 = final_4.len() > 0;
                                if (!f1 && f2) || (!f1 && !f2 && f3) || (!f1 && !f2 && !f3 && f4) {
                                    continue;
                                }
                                let mut moves = Vec::new();
                                moves.extend(hold);
                                moves.extend(rot);
                                moves.extend(shift);
                                if f1 {
                                    moves.push(GameMove::SoftDrop);
                                }
                                moves.extend(final_1);
                                if f2 {
                                    moves.push(GameMove::SoftDrop);
                                }
                                moves.extend(final_2);
                                if f3 {
                                    moves.push(GameMove::SoftDrop);
                                }
                                moves.extend(final_3);
                                if f4 {
                                    moves.push(GameMove::SoftDrop);
                                }
                                moves.extend(final_4);
                                moves.push(GameMove::HardDrop);
                                moves_list.push(moves);
                            }
                        }
                    }
                }
            }
        }
    }
    moves_list
});