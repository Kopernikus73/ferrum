// interaction.rs
//
// General interaction for ferrum (inner api)

use crate::{Fen, Field, FlagData, EvaluationScore, ChessMove};
use crate::constants::*;

use crate::board::{
    moves::find_legal_moves,
    manipulation::{
        generate_field_from_fen,
        // generate_fen_from_field,
        generate_field_from_move
    }
};
use crate::engine::eval::evaluate_single_position;


/// TODO! \
/// Finds the best possible move according to the evaluation sore from `evaluate_single_position()`
pub fn find_best_move(fen: Option<&Fen>) -> (ChessMove, EvaluationScore, FlagData){
    // Generate Field from FEN
    let (mut field , player_color, mut flag_data): (Field, u32, FlagData) = generate_field_from_fen(fen);

    //let mut flag_data: FlagData = 0b000000_000000_1_1_000000000000000000; // [6] moves_since_pawn, [6] en_passant_square, [1] castle queen, [1] castle queen, ?[9] total moves, [18/9?] unused
    let mut legal_moves: Vec<Vec<u32>> = Vec::with_capacity(64);

    for i in 0..64 {
        let chess_move = field[i];
        legal_moves.push(find_legal_moves(chess_move, &field, flag_data, player_color));
        println!("PIECE{}: {:?}",i, legal_moves[i]);
    }
    //println!("piece3:{:b}", field[3]);
    println!("\nfd: {:032b}\n",&flag_data);

    //println!("{:?}", legal_moves);


    // ########################
    // ## Evaluation Tactics ##
    // ########################

    // ## Single Depth - No Alpha-Beta pruning ##
    // Most simple Szenario: no a-b pruning, checking every single move till the end (here firstly only one move)

    let mut current_best_eval: EvaluationScore = evaluate_single_position(&field, &flag_data);
    let mut current_best_move: ChessMove = 0;
    println!("INIT Eval: {}", &current_best_eval);

    let mut elapsed_times: Vec<u128> = vec![];

    let elapsed_time_inner = std::time::Instant::now();
    match player_color{
        COLOR_WHITE => {
            for (piece_index, piece_moves) in legal_moves.iter().enumerate(){
                for chess_move in piece_moves{
                    elapsed_times.push(generate_field_from_move(&mut field, &mut flag_data, piece_index, *chess_move));
                    let eval = evaluate_single_position(&field, &flag_data);
                    if eval > current_best_eval {
                        current_best_eval = eval;
                        current_best_move = (chess_move << TO_SHIFT) | ((piece_index as u32) << FROM_SHIFT);
                        //println!("best_move: {:b} with eval: {}", &current_best_move, &current_best_eval);
                    }
                }
            }
        }
        COLOR_BLACK => {
            for (piece_index, piece_moves) in legal_moves.iter().enumerate(){
                for chess_move in piece_moves{
                    elapsed_times.push(generate_field_from_move(&mut field, &mut flag_data, piece_index, *chess_move));
                    let eval = evaluate_single_position(&field, &flag_data);
                    if eval < current_best_eval {
                        current_best_eval = eval;
                        current_best_move = (chess_move << TO_SHIFT) | ((piece_index as u32) << FROM_SHIFT);
                        //println!("best_move: {:b} with eval: {}", &current_best_move, &current_best_eval);
                    }
                }
            }
        }
        _ => {}
    }

    let mut added_time = 0;
    let numberoftimes = elapsed_times.len() as u128;
    for time in elapsed_times{
        added_time += time;
    }

    let final_time = added_time / numberoftimes;

    println!("\x1b[33mInner Time:\x1b[0m {:?}\n\x1b[33mFINISHED TIME:\x1b[0m {}ns | {}ns",elapsed_time_inner.elapsed() ,&final_time, added_time);

    // TODO!
    // If no move leads to a better position -> take first move since it wouldn't continue otherwise
    // Just as a fallback -> won't be needed with a more complex evaluation since little position changes can lead to big eval changes

    // Return the best move
    (current_best_move, current_best_eval, flag_data)
}