// moves.rs
//
// Generating possible moves from a given field
// Ignoring checkmate -> ignored by the evaluation engine (least possible eval)

use crate::{Piece, Field, FlagData, ChessMove};
use crate::constants::*;


/// Find all legal chess_moves a piece can make on the board
pub fn find_legal_moves(piece: Piece, field: &Field, flag_data: FlagData, player_color: u32) -> Vec<ChessMove> {
    let mut legal_moves: Vec<ChessMove> = Vec::new();

    // Piece Data
    // Only Position and Destination are shifted because they are translated to indices for the field array
    let color = piece & COLOR_MASK;
    let piece_type = piece & PIECE_MASK;
    let position: u32 = (piece & FROM_MASK) >> FROM_SHIFT;
    //let _destination = (piece & TO_MASK) >> TO_SHIFT;
    //let _promotion = piece & PROMOTE_MASK;


    //println!("Piece: {:b} | From: {:b} | Pos:{:x}",piece, piece & FROM_MASK, position);
    //println!("{:0b}--{:0b}->{},{}",&piece,&PIECE_MASK,piece_type==PIECE_PAWN >> PIECE_SHIFT,position);
    //println!("color: {} | piece_type: {:x} | Position: {:x}", color, piece_type, position);

    // Flag Data
    // let _moves_since_pawn = (flag_data & PAWN_MOVES_MASK) >> PAWN_MOVES_SHIFT;
    let en_passant_square = (flag_data & EN_PASSANT_MASK) >> EN_PASSANT_SHIFT;
    let castle_queen = (flag_data & CASTLE_QUEEN_MASK) >> CASTLE_QUEEN_SHIFT;
    let castle_king = (flag_data & CASTLE_KING_MASK) >> CASTLE_KING_SHIFT;

    if color == player_color{
        match piece_type {
            PIECE_NONE => {}
            // Voll bestimmt
            PIECE_PAWN => {
                // Kein over-/underflow check nötig → kann nicht am Rand stehen
                if color == COLOR_WHITE {
                    // move forward
                    if field[(position + 8) as usize] & PIECE_MASK == PIECE_NONE {
                        legal_moves.push(position + 8);
                    }
                    if position > 7 && position < 16 && (field[(position + 16) as usize] | field[(position + 8) as usize]) & PIECE_MASK == PIECE_NONE {
                        legal_moves.push(position + 16);
                    }

                    // capture
                    let right_squares: [u32; 8] = [7,15,23,31,39,47,55,63];
                    let left_squares: [u32; 8] = [0,8,16,24,32,40,48,56];

                    // left capture
                    if !left_squares.contains(&position) && (field[(position + 7) as usize] & PIECE_MASK != PIECE_NONE || field[(position + 7) as usize] & COLOR_MASK == COLOR_BLACK){
                        legal_moves.push(position + 7);
                    }

                    // right capture
                    if !right_squares.contains(&position) && (field[(position + 9) as usize] & PIECE_MASK != PIECE_NONE || field[(position + 9) as usize] & COLOR_MASK == COLOR_BLACK){
                        legal_moves.push(position + 9);
                    }

                    // En Passant
                    if position + 7 == en_passant_square {
                        legal_moves.push(position + 7);
                    }
                    if  position + 9 == en_passant_square {
                        legal_moves.push(position + 9);
                    }
                } else if color == COLOR_BLACK {
                    // move forward
                    if field[(position - 8) as usize] & PIECE_MASK == PIECE_NONE {
                        legal_moves.push(position - 8);
                    }
                    if position > 47 && position < 56 && (field[(position - 16) as usize] | field[(position - 8) as usize]) & PIECE_MASK == PIECE_NONE {
                        legal_moves.push(position - 16);
                    }

                    // capture
                    let right_squares: [u32; 8] = [7,15,23,31,39,47,55,63];
                    let left_squares: [u32; 8] = [0,8,16,24,32,40,48,56];

                    // left capture
                    if !right_squares.contains(&position) && (field[(position - 7) as usize] & PIECE_MASK != PIECE_NONE || field[(position - 7) as usize] & COLOR_MASK == COLOR_WHITE){
                        legal_moves.push(position - 7);
                    }

                    // right capture
                    if !left_squares.contains(&position) && (field[(position - 9) as usize] & PIECE_MASK != PIECE_NONE || field[(position - 9) as usize] & COLOR_MASK == COLOR_WHITE){
                        legal_moves.push(position - 9);
                    }
                    // En Passant
                    if position - 7 == en_passant_square {
                        legal_moves.push(position - 7);
                    }
                    if  position - 9 == en_passant_square {
                        legal_moves.push(position - 9);
                    }
                }
            }
            // Voll bestimmt
            PIECE_KNIGHT => {
                let right_knight_squares: [u32; 16] = [7,15,23,31,39,47,55,63,6,14,22,30,38,46,54,62];
                let left_knight_squares: [u32; 16]  = [0,8,16,24,32,40,48,56,1,9,17,25,33,41,49,57];
                let right_squares: [u32; 8]         = [7,15,23,31,39,47,55,63];
                let left_squares: [u32; 8]          = [0,8,16,24,32,40,48,56];

                // 2 vertical 1 horizontal
                // 2 up 1 left
                if  position < 48 && !left_squares.contains(&position) && (field[(position+15) as usize] & COLOR_MASK != player_color || field[(position+15) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position+15);
                }
                // 2 down 1 left
                if position > 15 && !left_squares.contains(&position) && (field[(position-17) as usize] & COLOR_MASK != player_color || field[(position-17) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position-17);
                }

                // 2 up 1 right
                if position < 48 && !right_squares.contains(&position) && (field[(position+17) as usize] & COLOR_MASK != player_color || field[(position+17) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position+17);
                }
                // 2 down 1 right
                if position > 15 && !right_squares.contains(&position) && (field[(position-15) as usize] & COLOR_MASK != player_color || field[(position-15) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position-15);
                }


                // 1 vertical 2 horizontal
                // 1 up 2 left
                if position < 56 && !left_knight_squares.contains(&position) && (field[(position+6) as usize] & COLOR_MASK != player_color || field[(position+6) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position+6);
                }
                // 1 down 2 left
                if position > 7 && !left_knight_squares.contains(&position) && (field[(position-10) as usize] & COLOR_MASK != player_color || field[(position-10) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position-10);
                }
                // 1 up 2 right
                if position < 56 && !right_knight_squares.contains(&position) && (field[(position+10) as usize] & COLOR_MASK != player_color || field[(position+10) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position+10);
                }
                // 1 up 2 right
                if position > 7 && !right_knight_squares.contains(&position) && (field[(position-6) as usize] & COLOR_MASK != player_color || field[(position-6) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position-6);
                }
            }
            // Voll bestimmt
            PIECE_BISHOP => {
                let right_squares: [u32; 8]          = [7,15,23,31,39,47,55,63];
                let left_squares: [u32; 8]         = [0,8,16,24,32,40,48,56];

                // up-left
                for move_length in 1..8{
                    if !(position<56) || (position+(7*move_length)) > 56 || left_squares.contains(&(position+7*(move_length-1))){
                        break
                    }
                    if !left_squares.contains(&position) && field[(position+(7*move_length)) as usize] & PIECE_MASK == PIECE_NONE{
                        legal_moves.push(position+(7*move_length));
                    } else if !left_squares.contains(&position) && field[(position+(7*move_length)) as usize] & COLOR_MASK != player_color{
                        legal_moves.push(position+(7*move_length));
                        break
                    } else {
                        break
                    }
                    if left_squares.contains(&(position+(7*move_length))){
                        break
                    }
                }
                // up-right
                for move_length in 1..8{
                    if !(position<56) || (position+(9*move_length)) > 56 || right_squares.contains(&(position+9*(move_length-1))){
                        break
                    }
                    if !right_squares.contains(&position) && field[(position+(9*move_length)) as usize] & PIECE_MASK == PIECE_NONE{
                        legal_moves.push(position+(9*move_length));
                    } else if !right_squares.contains(&position) && field[(position+(9*move_length)) as usize] & COLOR_MASK != player_color{
                        legal_moves.push(position+(9*move_length));
                        break
                    } else {
                        break
                    }
                    if right_squares.contains(&(position+(9*move_length))){
                        break
                    }
                }
                // down-left
                for move_length in 1..8{
                    if !(position>8) || (position-(9*move_length)) < 8 || left_squares.contains(&(position-9*(move_length-1))){
                        break
                    }
                    if !left_squares.contains(&position) && field[(position-(9*move_length)) as usize] & PIECE_MASK == PIECE_NONE{
                        legal_moves.push(position-(9*move_length));
                    } else if !left_squares.contains(&position) && field[(position-(9*move_length)) as usize] & COLOR_MASK != player_color{
                        legal_moves.push(position-(9*move_length));
                        break
                    } else {
                        break
                    }
                    if left_squares.contains(&(position-(9*move_length))){
                        break
                    }

                }
                // down-right
                for move_length in 1..8{
                    if !(position>8) || (position-(7*move_length)) < 8 || right_squares.contains(&(position-7*(move_length-1))){
                        break
                    }
                    //println!("{} && {} -> {}",!left_squares.contains(&position),field[(position-(7*move_length)) as usize] & PIECE_MASK == PIECE_NONE, field[(position-(7*move_length)) as usize]);
                    if !right_squares.contains(&position) && field[(position-(7*move_length)) as usize] & PIECE_MASK == PIECE_NONE {
                        legal_moves.push(position-(7*move_length));
                    } else if !right_squares.contains(&position) && field[(position-(7*move_length)) as usize] & COLOR_MASK != player_color{
                        legal_moves.push(position-(7*move_length));
                        break
                    } else {
                        break
                    }
                    if right_squares.contains(&(position-(7*move_length))){
                        break
                    }
                }
            }
            // Voll bestimmt
            PIECE_ROOK => {
                let right_squares: [u32; 8]          = [7,15,23,31,39,47,55,63];
                let left_squares: [u32; 8]         = [0,8,16,24,32,40,48,56];

                for move_length in 1..8 {
                    // left
                    if left_squares.contains(&(position-(move_length-1))) || left_squares.contains(&position){
                        break
                    }

                    if field[(position-move_length) as usize] & PIECE_MASK == PIECE_NONE{
                        legal_moves.push(position-move_length);
                    } else if (field[(position-move_length) as usize] & COLOR_MASK) != player_color{
                        legal_moves.push(position-move_length);
                        break
                    } else{
                        break
                    }
                }

                for move_length in 1..8 {
                    // right
                    if right_squares.contains(&(position+(move_length-1))) || right_squares.contains(&position){
                        break
                    }

                    if field[(position+move_length) as usize] & PIECE_MASK == PIECE_NONE{
                        legal_moves.push(position+move_length);
                    } else if (field[(position+move_length) as usize] & COLOR_MASK) != player_color{
                        legal_moves.push(position+move_length);
                        break
                    } else{
                        break
                    }
                }

                for move_length in 1..8 {
                    // up
                    if (position+(move_length-1)*8) > 55{
                        break
                    }

                    if field[(position+move_length*8) as usize] & PIECE_MASK == PIECE_NONE{
                        legal_moves.push(position+move_length*8);
                    } else if field[(position+move_length*8) as usize] & COLOR_MASK != player_color{
                        legal_moves.push(position+move_length*8);
                        break
                    } else {
                        break
                    }
                }

                for move_length in 1..8 {
                    // down
                    if (position-(move_length-1)*8) < 8{
                        break
                    }

                    if field[(position-move_length*8) as usize] & PIECE_MASK == PIECE_NONE{
                        legal_moves.push(position-move_length*8);
                    } else if field[(position-move_length*8) as usize] & COLOR_MASK != player_color{
                        legal_moves.push(position-move_length*8);
                        break
                    } else {
                        break
                    }
                }
            }
            // Voll bestimmt
            PIECE_QUEEN => {
                // Queen moves
                let right_squares: [u32; 8]          = [7,15,23,31,39,47,55,63];
                let left_squares: [u32; 8]         = [0,8,16,24,32,40,48,56];

                // from bishop
                // up-left
                for move_length in 1..8{
                    if !(position<56) || (position+(7*move_length)) > 56 || left_squares.contains(&(position+7*(move_length-1))){
                        break
                    }
                    if !left_squares.contains(&position) && field[(position+(7*move_length)) as usize] & PIECE_MASK == PIECE_NONE{
                        legal_moves.push(position+(7*move_length));
                    } else if !left_squares.contains(&position) && field[(position+(7*move_length)) as usize] & COLOR_MASK != player_color{
                        legal_moves.push(position+(7*move_length));
                        break
                    } else {
                        break
                    }
                    if left_squares.contains(&(position+(7*move_length))){
                        break
                    }
                }
                // up-right
                for move_length in 1..8{
                    if !(position<56) || (position+(9*move_length)) > 56 || right_squares.contains(&(position+9*(move_length-1))){
                        break
                    }
                    if !right_squares.contains(&position) && field[(position+(9*move_length)) as usize] & PIECE_MASK == PIECE_NONE{
                        legal_moves.push(position+(9*move_length));
                    } else if !right_squares.contains(&position) && field[(position+(9*move_length)) as usize] & COLOR_MASK != player_color{
                        legal_moves.push(position+(9*move_length));
                        break
                    } else {
                        break
                    }
                    if right_squares.contains(&(position+(9*move_length))){
                        break
                    }
                }
                // down-left
                for move_length in 1..8{
                    if !(position>8) || (position-(9*move_length)) < 8 || left_squares.contains(&(position-9*(move_length-1))){
                        break
                    }
                    if !left_squares.contains(&position) && field[(position-(9*move_length)) as usize] & PIECE_MASK == PIECE_NONE{
                        legal_moves.push(position-(9*move_length));
                    } else if !left_squares.contains(&position) && field[(position-(9*move_length)) as usize] & COLOR_MASK != player_color{
                        legal_moves.push(position-(9*move_length));
                        break
                    } else {
                        break
                    }
                    if left_squares.contains(&(position-(9*move_length))){
                        break
                    }

                }
                // down-right
                for move_length in 1..8{
                    if !(position>8) || (position-(7*move_length)) < 8 || right_squares.contains(&(position-7*(move_length-1))){
                        break
                    }
                    //println!("{} && {} -> {}",!left_squares.contains(&position),field[(position-(7*move_length)) as usize] & PIECE_MASK == PIECE_NONE, field[(position-(7*move_length)) as usize]);
                    if !right_squares.contains(&position) && field[(position-(7*move_length)) as usize] & PIECE_MASK == PIECE_NONE {
                        legal_moves.push(position-(7*move_length));
                    } else if !right_squares.contains(&position) && field[(position-(7*move_length)) as usize] & COLOR_MASK != player_color{
                        legal_moves.push(position-(7*move_length));
                        break
                    } else {
                        break
                    }
                    if right_squares.contains(&(position-(7*move_length))){
                        break
                    }
                }

                // from rook
                let right_squares: [u32; 8]          = [7,15,23,31,39,47,55,63];
                let left_squares: [u32; 8]         = [0,8,16,24,32,40,48,56];

                for move_length in 1..8 {
                    // left
                    if left_squares.contains(&(position-(move_length-1))) || left_squares.contains(&position){
                        break
                    }

                    if field[(position-move_length) as usize] & PIECE_MASK == PIECE_NONE{
                        legal_moves.push(position-move_length);
                    } else if (field[(position-move_length) as usize] & COLOR_MASK) != player_color{
                        legal_moves.push(position-move_length);
                        break
                    } else{
                        break
                    }
                }

                for move_length in 1..8 {
                    // right
                    if right_squares.contains(&(position+(move_length-1))) || right_squares.contains(&position){
                        break
                    }

                    if field[(position+move_length) as usize] & PIECE_MASK == PIECE_NONE{
                        legal_moves.push(position+move_length);
                    } else if (field[(position+move_length) as usize] & COLOR_MASK) != player_color{
                        legal_moves.push(position+move_length);
                        break
                    } else{
                        break
                    }
                }

                for move_length in 1..8 {
                    // up
                    if (position+(move_length-1)*8) > 55{
                        break
                    }

                    if field[(position+move_length*8) as usize] & PIECE_MASK == PIECE_NONE{
                        legal_moves.push(position+move_length*8);
                    } else if field[(position+move_length*8) as usize] & COLOR_MASK != player_color{
                        legal_moves.push(position+move_length*8);
                        break
                    } else {
                        break
                    }
                }

                for move_length in 1..8 {
                    // down
                    if (position-(move_length-1)*8) < 8{
                        break
                    }

                    if field[(position-move_length*8) as usize] & PIECE_MASK == PIECE_NONE{
                        legal_moves.push(position-move_length*8);
                    } else if field[(position-move_length*8) as usize] & COLOR_MASK != player_color{
                        legal_moves.push(position-move_length*8);
                        break
                    } else {
                        break
                    }
                }
            }
            // Voll bestimmt
            PIECE_KING => {
                let right_squares: [u32; 8]          = [7,15,23,31,39,47,55,63];
                let left_squares: [u32; 8]         = [0,8,16,24,32,40,48,56];

                //right
                if !right_squares.contains(&position) && (field[(position+1) as usize] & PIECE_MASK == PIECE_NONE || field[(position+1) as usize] & COLOR_MASK != player_color){
                    legal_moves.push(position+1);
                }

                //left
                if !left_squares.contains(&position) && (field[(position-1) as usize] & PIECE_MASK == PIECE_NONE || field[(position-1) as usize] & COLOR_MASK != player_color){
                    legal_moves.push(position-1);
                }

                //up
                if position < 56 && (field[(position+8) as usize] & PIECE_MASK == PIECE_NONE || field[(position+8) as usize] & COLOR_MASK != player_color){
                    legal_moves.push(position+8);
                }

                //down
                if position > 7 && (field[(position-8) as usize] & PIECE_MASK == PIECE_NONE || field[(position-8) as usize] & COLOR_MASK != player_color){
                    legal_moves.push(position-8);
                }

                //up-left
                if position < 56 && !left_squares.contains(&position) && (field[(position+7) as usize] & PIECE_MASK == PIECE_NONE || field[(position+7) as usize] & COLOR_MASK != player_color){
                    legal_moves.push(position+7);
                }

                //up-right
                if position < 56 && !right_squares.contains(&position) && (field[(position+9) as usize] & PIECE_MASK == PIECE_NONE || field[(position+9) as usize] & COLOR_MASK != player_color){
                    legal_moves.push(position+9);
                }

                //down-left
                if position > 7 && !left_squares.contains(&position) && (field[(position-9) as usize] & PIECE_MASK == PIECE_NONE || field[(position-9) as usize] & COLOR_MASK != player_color){
                    legal_moves.push(position-9);
                }

                //down-right
                if position > 7 && !right_squares.contains(&position) && (field[(position-7) as usize] & PIECE_MASK == PIECE_NONE || field[(position-7) as usize] & COLOR_MASK != player_color){
                    legal_moves.push(position-7);
                }

                // Castle left
                if castle_queen == 1 && (field[(position-1) as usize] | field[(position-2) as usize] | field[(position-3) as usize]) & PIECE_MASK == PIECE_NONE{
                    legal_moves.push(position-3);
                }

                // Castle right
                if castle_king == 1 && (field[(position+1) as usize] | field[(position+2) as usize]) & PIECE_MASK == PIECE_NONE{
                    legal_moves.push(position+2);
                }

            }
            _ => {
                // unreachable (Als Sicherheit)
            }
        }
    }

    legal_moves
}
