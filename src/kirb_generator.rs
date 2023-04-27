use std::collections::HashSet;

use std::collections::VecDeque;
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use crate::condition::event::Conditions;
use crate::condition::event::EventConditions;
use crate::condition::event::ReqOrMin;
use crate::condition::event::ScoreTarget;
use crate::game::ActivePiece;
use crate::game::BagRandomizer;
use crate::game::Board;
use crate::game::Game;
use crate::game::MaybeActive;
use crate::game::Mino;
use crate::game::PieceType;
use crate::game::Queue;
use crate::position::Position;
use crate::position::Rotation;
use crate::position::Vector;
use crate::puzzle::Puzzle;
use crate::replay::Info;


fn generate_ending_board(rng: &mut ThreadRng) -> Board {

    let mut board = Board::new();
    for y in 0..21 {
        board.matrix[y] = [Some(Mino::Gray); 10];
    }
    
    let mut residue = *(0..=4).collect::<Vec<usize>>().choose(rng)
            .expect("Always elements to choose from");
    if residue == 0 {
        let all_clear = rng.gen_bool(0.33);
        if !all_clear {
            residue = 1;
        }
    }

    let mut columns = (0..10).collect::<Vec<usize>>();
    columns.shuffle(rng);
    let mut columns = columns.into_iter();

    for _ in 0..residue {
        let doubled_up = rng.gen_bool(0.25);
        if doubled_up {
            let col = columns.next()
                    .expect("Always enough columns to choose from");
            board.matrix[21][col] = Some(Mino::Gray);
            board.matrix[22][col] = Some(Mino::Gray);
        } else {
            let col_1 = columns.next()
                    .expect("Always enough columns to choose from");
            let col_2 = columns.next()
                    .expect("Always enough columns to choose from");
            board.matrix[21][col_1] = Some(Mino::Gray);
            board.matrix[21][col_2] = Some(Mino::Gray);
        }
    }

    board
}

fn try_remove(board: &Board, placement: &ActivePiece) -> Option<Board> {
    let mut board = board.clone();
    let mut lockout = true;
    for mino in placement.get_mino_positions() {
        let row = board.matrix.get_mut(mino.y as usize)?;
        let cell = row.get_mut(mino.x as usize)?;
        if cell.is_none() {
            return None;
        }
        if mino.y < 20 {
            lockout = false;
        }
        *cell = None;
    }
    if lockout {
        return None;
    }
    Some(board)    
}

fn can_place(board: &Board, placement: ActivePiece, irs: Rotation) -> bool {
    let mut queue = VecDeque::new();
    let mut reachable = HashSet::new();
    if let Some(active) = ActivePiece::spawn(board, placement.piece_type, irs) {
        queue.push_back(active.clone());
        reachable.insert(active);
    }

    while let Some(active) = queue.pop_front() {
        let mut options = Vec::new();
        for shift in [Vector::ONE_DOWN, Vector::ONE_LEFT, Vector::ONE_RIGHT] {
            let mut active = active.clone();
            if active.try_move(board, shift) {
                options.push(active);
            }
        }
        for rot in [Rotation::Anticlockwise, Rotation::Clockwise] {
            let mut active = active.clone();
            if active.try_rotate(board, rot).is_some() {
                options.push(active);
            }
        }
        for option in options.into_iter() {
            if !reachable.contains(&option) {
                queue.push_back(option.clone());
                reachable.insert(option);
            }
        }
    }

    reachable.contains(&placement)
}

fn can_spawn(board: &Board, piece: PieceType, irs: Rotation) -> bool {
    ActivePiece::spawn(board, piece, irs).is_some()
}

fn no_floating(board: &Board) -> bool {
    let mut lines_done = false;
    for y in 0..26 {
        if board.matrix[y].iter().all(|x| x.is_some()) {
            if lines_done {
                return false;
            }
        } else {
            lines_done = true;
        }
    }
    true
}

fn remove_piece(rng: &mut ThreadRng,
        board: Board, piece_type: PieceType, held: PieceType, req_no_irs: bool) -> Option<Board> {
    let mut options = Vec::new();
    for r in 0..4 {
        let rot = Rotation::from_num_cw(r);
        for y in 0..26 {
            for x in 0..10 {
                let pos = Position::new(x, y);
                let placement = ActivePiece {piece_type, pos, rot};
                let board_after = try_remove(&board, &placement);
                if let Some(board) = board_after {
                    let irs_list = [Rotation::Zero, Rotation::Anticlockwise, Rotation::Clockwise];
                    let mut iter = irs_list.into_iter().map(|irs| {
                        no_floating(&board) && can_spawn(&board, held, irs)
                                && placement == placement.get_ghost(&board)
                                && can_place(&board, placement.clone(), irs)
                    });
                    let is_valid = if req_no_irs {
                        iter.all(|b| b)
                    } else {
                        iter.any(|b| b)
                    };
                    if is_valid {
                        options.push(board);
                    }
                }
            }
        }
    }

    options.choose(rng).cloned()
}

fn skim_board(rng: &mut ThreadRng, board: &Board) -> Option<Board> {
    let mut board = board.clone();
    for row in (0..21).rev() {
        let skim = rng.gen_bool(0.1);
        if skim {
            let bottom = board.matrix[0];
            if !bottom.iter().all(|x| x.is_some()) {
                return None;
            }
            for y in 0..row {
                board.matrix[y] = board.matrix[y+1];
            }
            board.matrix[row] = bottom;
        }
    }

    Some(board)
}


fn remove_piece_skim(rng: &mut ThreadRng, board: Board, piece_type: PieceType,
        held: PieceType, req_no_irs: bool) -> Option<Board> {
    for _ in 0..10 {
        if let Some(board) = skim_board(rng, &board) {
            if let Some(option) = remove_piece(rng, board, piece_type, held, req_no_irs) {
                return Some(option);
            }
        }
    }
    None
}

fn generate_order(rng: &mut ThreadRng, queue: &[PieceType])
        -> Vec<(PieceType, PieceType, bool)> {
    let mut order = Vec::new();

    let mut iter = queue.iter().cloned();
    let mut hold = iter.next().expect("Should be at least one piece");
    let mut prev_held = false;
    for mut next in iter {
        let held = rng.gen_bool(0.5);
        if held {
            let temp = hold;
            hold = next;
            next = temp;
        }
        if held && !prev_held {
            order.push((next, hold, order.is_empty()));
        } else {
            order.push((next, next, order.is_empty()));
        }
        prev_held = held;
    }

    order
}

fn try_generate(rng: &mut ThreadRng, queue: &[PieceType]) -> Option<Board> {
    let order = generate_order(rng, queue);

    let mut board = generate_ending_board(rng);

    for (piece, held, req_no_irs) in order.iter().rev() {
        board = remove_piece_skim(rng, board, *piece, *held, *req_no_irs)?;
    }

    Some(board)
}

fn rate_board(board: &Board) -> usize {
    let mut score = 0;
    for y in 0..26 {
        for x in 0..10 {
            let pos = Position::new(x, y);
            for dir in [Vector::ONE_LEFT, Vector::ONE_RIGHT,
                    Vector::ONE_UP, Vector::ONE_DOWN] {
                if board[pos].is_some() && board[pos + dir].is_none() {
                    score += 2;
                }
            }
            if board[pos].is_some() && board[pos + Vector::ONE_DOWN].is_none() {
                score += 100;
            }
        }
    }
    score
}

fn default_board() -> Board {
    let mut board = Board::new();
    for y in 0..19 {
        board.matrix[y] = [Some(Mino::Gray); 10];
    }
    board.matrix[19][0] = Some(Mino::Gray);
    board.matrix[20][0] = Some(Mino::Gray);
    board
}

pub fn generate(difficulty: u32) -> Puzzle {
    let difficulty = (difficulty + 0) as usize;

    let mut rando = BagRandomizer::new();
    let mut info = Info::new();
    let mut queue = Vec::new();
    for _ in 0..difficulty {
        queue.push(rando.next(&mut info));
    }
    
    let mut rng = rand::thread_rng();
    let length = rng.gen_range(queue.len().min(6).min(difficulty)..=queue.len().min(difficulty));
    let complexity = (difficulty - length + 1).pow(2);
    let mut best_board = default_board();
    let mut best_score = 100000000;
    for _ in 0..100 {
        //println!("generate");
        if let Some(board) = try_generate(&mut rng, &queue[0..length]) {
            let score = rate_board(&board);
            let score = score.max(difficulty * 10 * complexity);
            if score < best_score {
                best_score = score;
                best_board = board;
            }
        } else {
            //println!("fail");
        }
    }

    let game = Game {
        piece: queue.get(1).map(|p| MaybeActive::Inactive(*p)),
        queue: Queue {
            length: 4,
            pieces: queue[2..].iter().cloned().collect(),
            rando: BagRandomizer::new(),
        },
        hold: queue.get(0).cloned(),
        has_held: false,
        board: best_board,
        in_zone: true,
        over: false,
    };
    Puzzle {
        game,
        win_conditions: Conditions {
            conditions: vec![
                EventConditions::Condition(
                    ScoreTarget {score: 0, target: 1},
                    crate::condition::event::EventScorer::ZoneClear {
                        req_lines: ReqOrMin::Min(21),
                    },
                ),
            ],
        },
        end_conditions: Conditions {
            conditions: vec![
                EventConditions::Condition(
                    ScoreTarget {score: 0, target: 1},
                    crate::condition::event::EventScorer::ZoneClear {
                        req_lines: ReqOrMin::Min(0),
                    },
                ),
            ],
        },
        won: false,
        over: false,
    }
}