// Flag Masks + Shifts
const PAWN_MOVES_MASK: u32 =
    0b111111_000000_00000000000000000000;
const EN_PASSANT_MASK: u32 =
    0b000000_111111_00000000000000000000;

const PAWN_MOVES_SHIFT: u32 = 26;
const EN_PASSANT_SHIFT: u32 = 20;



// Field Masks + Shifts
const COLOR_MASK: u32 =   0b1_000_000000_000000_0_00_0000000000000;
const PIECE_MASK: u32 =   0b0_111_000000_000000_0_00_0000000000000;
const FROM_MASK: u32 =    0b0_000_111111_000000_0_00_0000000000000;
const TO_MASK: u32 =      0b0_000_000000_111111_0_00_0000000000000;
const PROMOTE_MASK: u32 = 0b0_000_000000_000000_1_00_0000000000000;
const CHECK_MASK: u32 =   0b0_000_000000_000000_0_11_0000000000000;

//const COLOR_SHIFT: u32 =   31;
//const PIECE_SHIFT: u32 =   28;
const FROM_SHIFT: u32 =    22;
const TO_SHIFT: u32 =      16;
//const PROMOTE_SHIFT: u32 = 15;
//const CHECK_SHIFT: u32 =   13;


// Pieces
pub const PIECE_NONE:   u32 =
    0b0_000_000000_000000_0_00_0000000000000;
pub const PIECE_PAWN:   u32 =
    0b0_001_000000_000000_0_00_0000000000000;
pub const PIECE_KNIGHT: u32 =
    0b0_010_000000_000000_0_00_0000000000000;
pub const PIECE_BISHOP: u32 =
    0b0_011_000000_000000_0_00_0000000000000;
pub const PIECE_ROOK:   u32 =
    0b0_100_000000_000000_0_00_0000000000000;
pub const PIECE_ROOK_MOVED:   u32 =
    0b0_101_000000_000000_0_00_0000000000000;
pub const PIECE_QUEEN:  u32 =
    0b0_110_000000_000000_0_00_0000000000000;
pub const PIECE_KING:   u32 =
    0b0_111_000000_000000_0_00_0000000000000;

// Colors
pub const COLOR_WHITE: u32 =
    0b0_000_000000_000000_0_00_0000000000000;
pub const COLOR_BLACK: u32 =
    0b1_000_000000_000000_0_00_0000000000000;


// FEN
const MAX_COUNTER: u32 = 8;


// Types
type Fen = String;
type FlagData = u32;




/// !TODO \
/// Generates a field from the calculated chess move
pub fn generate_field_from_move(_chess_move: u32) -> [u32; 64] {
    [0; 64]
}
/// !TODO \
/// Generates a field using fen
pub fn generate_field(fen: Option<&Fen>) -> ([u32; 64], u32) {
    let mut field: [u32; 64] = [0; 64];
    let player_color: u32;

    match fen{
        None => {
            eprintln!("\x1b[31mNo fen given\x1b[0m");
            std::process::exit(10);
        }
        Some(_fen) => {
            // Standard Field //TODO!
            // === Weiße Figuren ===
            let w = COLOR_WHITE;
            let b = COLOR_BLACK;

            // Reihe 0
            field[0] = PIECE_ROOK   | w;
            field[1] = PIECE_KNIGHT | w;
            field[2] = PIECE_BISHOP | w;
            field[3] = PIECE_QUEEN  | w;
            field[4] = PIECE_KING   | w;
            field[5] = PIECE_BISHOP | w;
            field[6] = PIECE_KNIGHT | w;
            field[7] = PIECE_ROOK   | w;

            // Reihe 1 – weiße Bauern
            for i in 8..16 {
                field[i] = PIECE_PAWN | w | ((i as u32) << FROM_SHIFT);
            }

            // === Schwarze Figuren ===

            // Reihe 7
            field[56] = PIECE_ROOK   | b;
            field[57] = PIECE_KNIGHT | b;
            field[58] = PIECE_BISHOP | b;
            field[59] = PIECE_QUEEN  | b;
            field[60] = PIECE_KING   | b;
            field[61] = PIECE_BISHOP | b;
            field[62] = PIECE_KNIGHT | b;
            field[63] = PIECE_ROOK   | b;


            // Reihe 6 – schwarze Bauern
            for i in 48..56 {
                field[i] = PIECE_PAWN | b | ((i as u32) << FROM_SHIFT);
            }

            // Test Piece
            // field[48] = PIECE_PAWN   | w | (48u32 << FROM_SHIFT) ;

            player_color = COLOR_BLACK;
        }
    }

    // Return field and player_color since they are always initialized
    (field, player_color)
}
/// !TODO \
/// Generates FEN using a string
pub fn generate_fen(field: &[u32; 64], _flag_data: u32) -> String{

    let mut fen = String::new();
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

/// Find all legal chess_moves a piece can make on the board
fn find_legal_moves(piece: u32, field: &[u32; 64], flag_data: u32, player_color: u32) -> Vec<u32> {
    let mut legal_moves = Vec::new();

    // Piece Data
    // Only Position and Destination are shifted because they are translated to indices for the field array
    let color = piece & COLOR_MASK;
    let piece_type = piece & PIECE_MASK;
    let position = (piece & FROM_MASK) >> FROM_SHIFT;
    let _destination = (piece & TO_MASK) >> TO_SHIFT;
    let _promotion = piece & PROMOTE_MASK;
    let _check = piece & CHECK_MASK;


    //println!("Piece: {:b} | From: {:b} | Pos:{:x}",piece, piece & FROM_MASK, position);
    //println!("{:0b}--{:0b}->{},{}",&piece,&PIECE_MASK,piece_type==PIECE_PAWN >> PIECE_SHIFT,position);
    //println!("color: {} | piece_type: {:x} | Position: {:x}", color, piece_type, position);

    // Flag Data
    let _moves_since_pawn = (flag_data & PAWN_MOVES_MASK) >> PAWN_MOVES_SHIFT;
    let _en_passant_square = (flag_data & EN_PASSANT_MASK) >> EN_PASSANT_SHIFT;

    if color == player_color{
        match piece_type {
            PIECE_NONE => {}
            // Voll bestimmt
            PIECE_PAWN => {
                // Kein over-/underflow check nötig → kann nicht am Rand stehen
                if color == COLOR_WHITE {
                    // move forward
                    //println!("{:0b}",field[(position + 8) as usize]);
                    if field[(position + 8) as usize] & PIECE_MASK == PIECE_NONE {
                        legal_moves.push(position + 8);
                    }
                    if position > 7 && position < 16 && field[(position + 16) as usize] & PIECE_MASK == PIECE_NONE {
                        legal_moves.push(position + 16);
                    }

                    // capture
                    let left_squares: [u32; 8] = [7,15,23,31,39,47,55,63];
                    let right_squares: [u32; 8] = [0,8,16,24,32,40,48,56];

                    // right capture
                    if !right_squares.contains(&position) && field[(position + 7) as usize] & PIECE_MASK != PIECE_NONE && field[(position + 7) as usize] & COLOR_MASK == COLOR_BLACK{
                        legal_moves.push(position + 7);
                    }

                    // left capture
                    if !left_squares.contains(&position)  && field[(position + 9) as usize] & PIECE_MASK != PIECE_NONE && field[(position + 9) as usize] & COLOR_MASK == COLOR_BLACK {
                        legal_moves.push(position + 9);
                    }

                } else if color == COLOR_BLACK {
                    // move forward
                    //println!("{}", position);
                    if field[(position - 8) as usize] & PIECE_MASK == PIECE_NONE {
                        legal_moves.push(position - 8);
                    }
                    if position > 40 && position < 56 && field[(position - 16) as usize] & PIECE_MASK == PIECE_NONE {
                        legal_moves.push(position - 16);
                    }

                    // capture
                    let left_squares: [u32; 8] = [7,15,23,31,39,47,55,63];
                    let right_squares: [u32; 8] = [0,8,16,24,32,40,48,56];

                    // right capture
                    if !left_squares.contains(&position)  && field[(position - 7) as usize] & PIECE_MASK != PIECE_NONE && field[(position - 7) as usize] & COLOR_MASK == COLOR_WHITE{
                        legal_moves.push(position - 7);
                    }

                    // left capture
                    if !right_squares.contains(&position) && field[(position - 9) as usize] & PIECE_MASK != PIECE_NONE && field[(position - 9) as usize] & COLOR_MASK == COLOR_WHITE  {
                        legal_moves.push(position - 9);
                    }
                }
            }
            // Nicht bestimmt
            PIECE_KNIGHT => {
                // Knight moves
            }
            // Nicht bestimmt
            PIECE_BISHOP => {
                // Bishop moves
            }
            // Nicht bestimmt
            PIECE_ROOK | PIECE_ROOK_MOVED => {
                // Rook moves
            }
            // Nicht bestimmt
            PIECE_QUEEN => {
                // Queen moves
            }
            // Nicht bestimmt
            PIECE_KING => {
                // King moves
            }
            _ => {
                // unreachable (Als Sicherheit)
            }
        }
    }

    legal_moves
}

fn _evaluate_single_position() -> u32 {
    10
}

// Production Accessible functions (Other are available/public due to testing reasons)
pub fn find_best_move(fen: Option<&Fen>) -> (u32, FlagData){
    // Generate Field from FEN
    let (field , player_color): ([u32; 64], u32) = generate_field(fen);

    let flag_data: FlagData = 0b000001_001000_00000000000000000000; // [6] moves_since_pawn, [6] en_passant_square, [20] unused
    let mut legal_moves: Vec<Vec<u32>> = Vec::with_capacity(64);

    for i in 0..64 {
        let chess_move = field[i];
        legal_moves.push(find_legal_moves(chess_move, &field, flag_data, player_color));
    }

    println!("{:?}", legal_moves);
    //println!("{:?}", field);
    //println!("{:b}", field[8]);


    let best_move: (u32, FlagData) = (0, 0); // (value, fen, flag_data) -> For single depth

    best_move
}