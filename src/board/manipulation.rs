// manipulation.rs
//
// Board manipulation
// e.g. - Generating a field from input formats (FEN, PGN?)
//      - Changing a given board by a input move
//      - Generate a new format from a board (FEN)
//      - Generate a move by algebraic notation todo!
//      - Generate algebraic notation by a move todo!

use crate::{Fen, FlagData, Field};
use crate::constants::*;

macro_rules! p {
    (none) => {
        PIECE_NONE
    };
    (p, $color:tt, $spot:expr) => {
        PIECE_PAWN | pcolor!($color) | $spot << FROM_SHIFT
    };
    (b, $color:tt, $spot:expr) => {
        PIECE_BISHOP | pcolor!($color) | $spot << FROM_SHIFT
    };
    (n, $color:tt, $spot:expr) => {
        PIECE_KNIGHT | pcolor!($color) | $spot << FROM_SHIFT
    };
    (r, $color:tt, $spot:expr) => {
        PIECE_ROOK | pcolor!($color) | $spot << FROM_SHIFT
    };
    (q, $color:tt, $spot:expr) => {
        PIECE_QUEEN | pcolor!($color) | $spot << FROM_SHIFT
    };
    (k, $color:tt, $spot:expr) => {
        PIECE_KING | pcolor!($color) | $spot << FROM_SHIFT
    };
}
macro_rules! pcolor {
    (black) => {
        COLOR_BLACK
    };
    (white) => {
        COLOR_WHITE
    }
}


/// !TODO \
/// Generates a field using fen
pub fn generate_field_from_fen(fen: Option<&Fen>) -> (Field, u32, FlagData) {
    let mut field: Field = [0; 64];
    let player_color: u32;
    let mut flag_data: FlagData = 0b000000_000000_0_0_000000000_000000000; // [6] moves_since_pawn, [6] en_passant_square, [1] castle queen, [1] castle queen, ?[9] total moves, [18/9?] unused

    fn debug_field_binary(field: &Field, label: &str) {
        println!("{}:", label);
        print!("  ");
        for row in 0..8{
            for col in 0..8 {
                print!("{:04b} ", field[64 - (row * 8) - (8 - col)] >> 28);
            }
            print!("\n  ");
        }
        println!();
    }
    fn print_grid() {
        for row in (0..8).rev() {
            print!("   ");
            for col in 0..8 {
                let num = row * 8 + col;
                print!("{:>2}   ", num);
            }
            println!();
        }
    }

    match fen{
        None => {
            eprintln!("\x1b[31mNo fen given\x1b[0m");
            std::process::exit(10);
        }
        Some(fen) => {
            let fen_parts = fen.split(" ").collect::<Vec<&str>>();

            if fen_parts.len() != 6{
                eprintln!("\x1b[31mInvalid fen length\x1b[0m");
                std::process::exit(11);
            }
            println!("{:?}", fen_parts);

            // Field
            let mut current_position: u32 = 56;
            let mut temp_slash = false;
            for piece_part in fen_parts[0].chars() {
                match piece_part {
                    'r' => {
                        field[current_position as usize] = p!(r, black, current_position);
                        current_position += 1;
                    }
                    'n' => {
                        field[current_position as usize] = p!(n, black, current_position);
                        current_position += 1;
                    }
                    'b' => {
                        field[current_position as usize] = p!(b, black, current_position);
                        current_position += 1;
                    }
                    'q' => {
                        field[current_position as usize] = p!(q, black, current_position);
                        current_position += 1;
                    }
                    'k' => {
                        field[current_position as usize] = p!(k, black, current_position);
                        current_position += 1;
                    }
                    'p' => {
                        field[current_position as usize] = p!(p, black, current_position);
                        current_position += 1;
                    }
                    'R' => {
                        field[current_position as usize] = p!(r, white, current_position);
                        current_position += 1;
                    }
                    'N' => {
                        field[current_position as usize] = p!(n, white, current_position);
                        current_position += 1;
                    }
                    'B' => {
                        field[current_position as usize] = p!(b, white, current_position);
                        current_position += 1;
                    }
                    'Q' => {
                        field[current_position as usize] = p!(q, white, current_position);
                        current_position += 1;
                    }
                    'K' => {
                        field[current_position as usize] = p!(k, white, current_position);
                        current_position += 1;
                    }
                    'P' => {
                        field[current_position as usize] = p!(p, white, current_position);
                        current_position += 1;
                    }
                    '/' => {
                        temp_slash = true;
                    }
                    _ => {
                        if let Some(free_spaces) = piece_part.to_digit(10) {
                            for field_index in 0..free_spaces {
                                field[(current_position + field_index) as usize] = p!(none);
                            }
                            current_position += free_spaces;
                        } else {
                            eprintln!("\x1b[31mInvalid fen piece character\x1b[0m");
                            std::process::exit(12);
                        }
                    }
                }
                // position logic
                if current_position % 8 == 0 && current_position > 15 && !temp_slash {
                    current_position -= 16;
                } else if temp_slash{
                    temp_slash = false;
                }
                //println!("{} - {}", current_position, piece_part);
                //debug_field_binary(&field, &piece_part.to_string());
            }
            debug_field_binary(&field, "Field");
            print_grid();
            println!();

            // Player Color
            player_color = match fen_parts[1]{
                "w" => {COLOR_WHITE},
                "b" => {COLOR_BLACK},
                _ => {
                    eprintln!("\x1b[31mInvalid fen color character\x1b[0m");
                    std::process::exit(13);
                }
            };

            // Flag Data
            for castle_part in fen_parts[2].chars() {
                match castle_part{
                    'q' | 'Q' => {
                        flag_data += 0b1 << (CASTLE_QUEEN_SHIFT-1);
                    }
                    'k' | 'K' => {
                        flag_data += 0b1 << (CASTLE_KING_SHIFT-1);
                    }
                    '-' => {

                    }
                    _ => {
                        eprintln!("\x1b[31mInvalid fen castle character\x1b[0m");
                        std::process::exit(14);
                    }
                }
            }

            // TODO! EN PASSANT + ALGEBRAIC PARSER

            if let Ok(moves_since_pawn) = fen_parts[4].parse::<u32>(){
                flag_data += moves_since_pawn << PAWN_MOVES_SHIFT;
            } else{
                eprintln!("\x1b[31mInvalid fen moves since last pawn move number\x1b[0m");
                std::process::exit(15);
            }

            if let Ok(total_moves) = fen_parts[5].parse::<u32>(){
                flag_data += (total_moves - 1) << TOTAL_MOVES_SHIFT; // -1 because these are total moves and not the current move number as in FEN notation
            } else{
                eprintln!("\x1b[31mInvalid fen total moves number\x1b[0m");
                std::process::exit(16);
            }
        }
    }
    // FOR MANUAL CONTROL
    /*field = [
        p!(r, white, 0),p!(none),p!(b, white, 2),p!(k, white, 3),p!(q, white, 4),p!(b, white, 5),p!(none),p!(r, white, 7),
        p!(none),p!(none),p!(p, white, 10),p!(none),p!(p, white, 12),p!(p, white, 13),p!(p, white, 14),p!(none),
        p!(p, white, 16),p!(none),p!(n, white, 18),p!(none),p!(none),p!(n, white, 21),p!(none),p!(none),
        p!(none),p!(p, white, 25),p!(none),p!(p, white, 27),p!(none),p!(p, black, 29),p!(none),p!(p, white, 31),
        p!(p, black, 32),p!(p, black, 33),p!(none),p!(none),p!(none),p!(none),p!(none),p!(none),
        p!(none),p!(none),p!(none),p!(p, black, 43),p!(none),p!(none),p!(none),p!(none),
        p!(none),p!(none),p!(p, black, 50),p!(none),p!(p, black, 52),p!(none),p!(p, black, 54),p!(p, black, 55),
        p!(r, black, 56),p!(n, black, 57),p!(b, black, 58),p!(k, black, 59),p!(q, black, 60),p!(b, black, 61),p!(n, black, 62),p!(r, black, 63),
    ];
    player_color = pcolor!(white);
    */

    println!("fd: {:032b} (moves_since_pawn: {:06b}, en_passant_square: {:06b}, CastleQ: {:01b}, CastleK: {:01b})", flag_data, (flag_data & PAWN_MOVES_MASK) >> PAWN_MOVES_SHIFT, (flag_data & EN_PASSANT_MASK) >> EN_PASSANT_SHIFT, (flag_data & CASTLE_QUEEN_MASK) >> CASTLE_QUEEN_SHIFT, (flag_data & CASTLE_KING_MASK) >> CASTLE_KING_SHIFT);

    // Return field and player_color since they are always initialized
    (field, player_color, flag_data)
}

#[allow(dead_code)]
/// !TODO \
/// Generates FEN using a string
pub fn generate_fen_from_field(field: &Field, _flag_data: u32) -> Fen{
    let mut fen = Fen::new();
    let mut used_squares_counter: u32 = 0;
    let mut squares_counter: u32 = 0;
    for i in field{
        let to_add: char = ' ';
        match i {
            &PIECE_PAWN => {

            }
            _ => squares_counter += 1
        }

        used_squares_counter += 1;

        if to_add != ' '{
            match char::from_u32(squares_counter+48){
                None => {std::process::exit(50)}
                Some(c) =>{fen.push(c)}
            }
            squares_counter = 0;
            fen.push(to_add);
        }
        if used_squares_counter == MAX_COUNTER{
            match char::from_u32(squares_counter+48){
                None => {std::process::exit(50)}
                Some(c) =>{fen.push(c)}
            }
            fen.push('/');

            if squares_counter != 0{
                squares_counter = 0;
            }
            used_squares_counter = 0;
        }
    }
    fen
}

/// TODO! \
/// Generates a field from the calculated chess move
pub fn change_field_by_move(field: &mut Field, flag_data: &mut FlagData, from_usize: usize, to_u32: u32) -> u128 {
    let time_check = std::time::Instant::now();
    let _from_u32 = from_usize as u32;
    let to_usize = to_u32 as usize;

    //println!("Field old : {:?}", &field);
    //println!("{}: {:b}| {}: {:b}", &from_shifted, &field[from_shifted], &to_shifted, &field[to_shifted]);

    // get and remove old piece
    let piece_to_move = field[from_usize];
    field[from_usize] = PIECE_NONE;

    // Adjust flag_data
    if piece_to_move & PIECE_MASK == PIECE_PAWN{
        *flag_data += 1u32 << PAWN_MOVES_SHIFT;
    }

    // place new piece
    field[to_usize] = piece_to_move & !FROM_MASK | (to_u32 << TO_SHIFT);

    // castling check   // flag data?
    if piece_to_move & PIECE_MASK == PIECE_KING {
        if from_usize as i32 - to_u32 as i32 >= 2{
            field[to_usize+1] = PIECE_ROOK | (piece_to_move & COLOR_MASK) | ((to_u32+1) << TO_SHIFT);
            // delete
            field[to_usize-2] = PIECE_NONE;
        } else if from_usize as i32 - to_u32 as i32 <= -2{
            field[to_usize-1] = PIECE_ROOK | (piece_to_move & COLOR_MASK) | ((to_u32-1) << TO_SHIFT);
            // delete
            field[to_usize+1] = PIECE_NONE;
        }
    }

    //println!("{}: {:b}| {}: {:b}", &from_shifted, &field[from_shifted], &to_shifted, &field[to_shifted]);
    //println!("Field new : {:?}", &field);
    time_check.elapsed().as_nanos()
}
