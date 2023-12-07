
use enumset::EnumSet;
use rand::prelude::RngCore;

use crate::condition::all_clear::AllClearType;
use crate::condition::event::Conditions;
use crate::condition::event::EventConditions;
use crate::condition::event::ReqOrMin;
use crate::condition::event::ScoreTarget;
use crate::condition::spin::SpinConditions;
use crate::condition::spin::SpinHandler;
use crate::condition::spin::SpinScorer;
use crate::game::BagRandomizer;
use crate::game::Game;
use crate::game::PieceType;
use crate::game::Queue;
use crate::puzzle::Puzzle;

const NUM_BAGS: usize = 25;

pub fn generate() -> Puzzle {
    let non_t_options: Vec<PieceType> = EnumSet::all().into_iter()
        .filter(|p| *p != PieceType::T)
        .collect();

    let mut rng = rand::thread_rng();

    let mut pieces = Vec::new();
    for _ in 0..NUM_BAGS {
        for _ in 0..non_t_options.len() {
            let choice = (rng.next_u64() as usize) % non_t_options.len();
            pieces.push(non_t_options[choice]);
        }
        pieces.push(PieceType::T);
    }

    let mut game = Game::default();
    game.queue = Queue {
        length: 4,
        pieces: pieces.into(),
        rando: BagRandomizer::new(),
    };
    
    Puzzle {
        game,
        win_conditions: Conditions {
            conditions: vec![
                EventConditions::TSpinContext(
                    SpinHandler::new(None),
                    vec![
                        SpinConditions::Condition(
                            ScoreTarget { score: 0, target: 20 },
                            SpinScorer::LineClear {
                                req_lines: ReqOrMin::Req(2),
                                req_piece: Some(PieceType::T),
                                req_all_clear: AllClearType::NONE,
                                req_spin: Some(None),
                                req_hard: None,
                                negate: false
                            }
                        )
                    ]
                ),
            ],
        },
        end_conditions: Conditions {
            conditions: vec![
                EventConditions::TSpinContext(
                    SpinHandler::new(None),
                    vec![
                        SpinConditions::Condition(
                            ScoreTarget { score: 0, target: 1 },
                            SpinScorer::LineClear {
                                req_lines: ReqOrMin::Req(2),
                                req_piece: Some(PieceType::T),
                                req_all_clear: AllClearType::NONE,
                                req_spin: Some(None),
                                req_hard: None,
                                negate: true
                            }
                        )
                    ]
                ),
            ],
        },
        won: false,
        over: false,
    }
}