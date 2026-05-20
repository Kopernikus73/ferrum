// eval.rs
//
// Evaluation of positions (in combination with e.g. heuristics.rs)

use rand::RngExt;
use crate::{Piece, Field, FlagData, EvaluationScore};
use crate::constants::*;

/// TODO! \
/// Evaluates a single position (should be deterministic - for now it's random to get some data)
pub fn evaluate_single_position(field: &Field, _flag_data: &FlagData) -> EvaluationScore {


    let piece_value = move |piece: &Piece| -> EvaluationScore{
        let piece = piece & PIECE_MASK;
        // Normal Chess Piece Value (These may be multiples later)
        match piece {
            PIECE_PAWN => 1,
            PIECE_KNIGHT | PIECE_BISHOP => 3,
            PIECE_ROOK => 5,
            PIECE_QUEEN => 9,

            // No Value
            PIECE_KING => 0,
            PIECE_NONE | _  => 0
        }
    };

    let mut white_evaluation_score: EvaluationScore = 0;
    let mut black_evaluation_score: EvaluationScore = 0;

    let mut rng = rand::rng();
    white_evaluation_score += rng.random_range(0..10);

    // Add piece value to evaluation score
    for piece in field{
        match piece & COLOR_MASK{
            COLOR_WHITE => white_evaluation_score += piece_value(&piece),
            COLOR_BLACK | _ => black_evaluation_score += piece_value(&piece),
        }
    }

    // return
    white_evaluation_score - black_evaluation_score
}