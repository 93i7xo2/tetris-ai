use crate::api::json::JsonAiRes;
use crate::model::{Bag, ChildState, Game, GameMove, GameMoveRes, BOARD_HEIGHT, MOVES_2F};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum AiRes {
    Success {
        moves: Vec<GameMove>,
        score: Option<f64>,
    },
    Fail {
        reason: String,
    },
}
impl Display for AiRes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AiRes::Success { moves, score } => {
                let score = match score {
                    Some(score) => format!("{:.2}", score),
                    None => String::from("None"),
                };
                let moves = moves
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "Eval Score: {} Moves: [{}]", score, moves)
            }
            AiRes::Fail { reason } => {
                write!(f, "Eval Failed: {}", reason)
            }
        }
    }
}

pub trait Ai {
    fn evaluate(&mut self, game: &Game) -> AiRes;
    fn api_evaluate(&mut self, req: String) -> String {
        let game = match Game::from_str(&req) {
            Ok(game) => game,
            Err(parse_err) => {
                let output = JsonAiRes::Fail {
                    success: false,
                    reason: format!("Invalid request: {}", parse_err),
                };
                return serde_json::to_string(&output).unwrap();
            }
        };
        let res = self.evaluate(&game);
        res.to_json()
    }
    /// A quick and easy way to watch an ai play a game
    fn watch_ai(&mut self, seed: u64) {
        let mut bag = Bag::new(seed);
        let mut game = Game::from_bag_shuffled(&mut bag);
        println!("{}\n", game);
        'l: loop {
            let start = Instant::now();
            let res = self.evaluate(&game);
            let elapsed = start.elapsed();
            match res {
                AiRes::Success { moves, score } => {
                    for game_move in &moves {
                        if let GameMove::HardDrop = game_move {
                            let res = game.make_move(*game_move);
                            if let GameMoveRes::SuccessDrop(drop_res) = res {
                                if drop_res.top_out {
                                    println!("TOP OUT");
                                    break 'l;
                                }
                            }
                        } else {
                            game.make_move(*game_move);
                        }
                    }
                    let score = score
                        .map(|x| format!("{:.2}", x))
                        .unwrap_or(String::from("None"));
                    let moves = moves
                        .into_iter()
                        .map(|x| format!("{}", x))
                        .collect::<Vec<_>>()
                        .join(", ");
                    println!(
                        "{0}\nEval: {1} in {3:?}\n[{2}]\n",
                        game, score, moves, elapsed
                    );
                }
                AiRes::Fail { reason } => {
                    println!("Evaluation failed: {}", reason);
                    break;
                }
            }
            game.refill_queue_shuffled(&mut bag);
        }
    }
    /// Easy way to benchmark the performance of an Ai
    fn bench_ai(&mut self, eval_count: u32, seed: u64) {
        let mut bag = Bag::new(seed);
        let mut game = Game::from_bag_shuffled(&mut bag);

        let mut total_time = Duration::new(0, 0);

        for _ in 0..eval_count {
            let start = Instant::now();
            let res = self.evaluate(&game);
            let elapsed = start.elapsed();
            total_time += elapsed;

            match res {
                AiRes::Success { moves, .. } => {
                    for game_move in moves {
                        game.make_move(game_move);
                    }
                    game.refill_queue_shuffled(&mut bag);
                }
                AiRes::Fail { .. } => {
                    // Reset game
                    game = Game::from_bag_shuffled(&mut bag);
                }
            }
        }
        println!("Total evaluations: {}", eval_count);
        println!("Total time: {:?}", total_time);
        println!("Time per evaluation: {:?}", total_time / eval_count);
    }
}

pub struct SimpleAi;
impl SimpleAi {
    pub fn new() -> Self {
        SimpleAi
    }
}
impl Ai for SimpleAi {
    fn evaluate(&mut self, game: &Game) -> AiRes {
        let child_states = game.child_states(&MOVES_2F);
        let mut best_moves = None;
        let mut best_height = BOARD_HEIGHT;
        for child_state in child_states.iter() {
            let ChildState { game, moves } = child_state;
            let max_height = game.board.height_map.iter().fold(0, |a, b| a.max(*b)) as i32;
            if max_height < best_height {
                best_height = max_height;
                best_moves = Some(*moves);
            }
        }
        match best_moves {
            Some(moves) => AiRes::Success {
                moves: Vec::from(moves),
                score: Some(child_states.len() as f64),
            },
            None => AiRes::Fail {
                reason: "No valid moves".into(),
            },
        }
    }
}
