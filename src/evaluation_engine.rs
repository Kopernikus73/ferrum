// Flag Masks + Shifts
const PAWN_MOVES_MASK: u32 =
    0b111111_000000_00000000000000000000;
const EN_PASSANT_MASK: u32 =
    0b000000_111111_00000000000000000000;

const PAWN_MOVES_SHIFT: u32 = 26;
const EN_PASSANT_SHIFT: u32 = 20;



// Field Masks + Shifts
const COLOR_MASK: u32 =    0b1_000_000000_000000_0_00_0000000000000;
const PIECE_MASK: u32 =    0b0_111_000000_000000_0_00_0000000000000;
pub const FROM_MASK: u32 = 0b0_000_111111_000000_0_00_0000000000000;
pub const TO_MASK: u32 =   0b0_000_000000_111111_0_00_0000000000000;
const PROMOTE_MASK: u32 =  0b0_000_000000_000000_1_00_0000000000000;
const CHECK_MASK: u32 =    0b0_000_000000_000000_0_11_0000000000000;

//const COLOR_SHIFT: u32 =   31;
//const PIECE_SHIFT: u32 =   28;
pub const FROM_SHIFT: u32 =  22;
pub const TO_SHIFT: u32 =    16;
//const PROMOTE_SHIFT: u32 = 15;
//const CHECK_SHIFT: u32 =   13;

const FROM_TO_SHIFT: u32 =  6;

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
#[allow(dead_code)] // TODO!
const MAX_COUNTER: u32 = 8;


// Types
pub type Fen = String;
pub type FlagData = u32;
pub type Field = [u32; 64];
pub type ChessMove = u32;
pub type Piece = u32;
pub type EvaluationScore = i16;


/// !TODO \
/// Generates a field using fen
pub fn generate_field_from_fen(fen: Option<&Fen>) -> (Field, u32) {
    let mut field: Field = [0; 64];
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
            field[0] = PIECE_ROOK   | w | (0 << FROM_SHIFT);
            field[1] = PIECE_KNIGHT | w | (1 << FROM_SHIFT);
            field[2] = PIECE_BISHOP | w | (2 << FROM_SHIFT);
            field[3] = PIECE_QUEEN  | w | (3 << FROM_SHIFT);
            field[4] = PIECE_KING   | w | (4 << FROM_SHIFT);
            field[5] = PIECE_BISHOP | w | (5 << FROM_SHIFT);
            field[6] = PIECE_KNIGHT | w | (6 << FROM_SHIFT);
            field[7] = PIECE_ROOK   | w | (7 << FROM_SHIFT);

            // Reihe 1 – weiße Bauern
            for i in 8..16 {
                field[i] = PIECE_PAWN | w | ((i as u32) << FROM_SHIFT);
            }

            // === Schwarze Figuren ===

            // Reihe 7
            field[56] = PIECE_ROOK   | b | (56 << FROM_SHIFT);
            field[57] = PIECE_KNIGHT | b | (57 << FROM_SHIFT);
            field[58] = PIECE_BISHOP | b | (58 << FROM_SHIFT);
            field[59] = PIECE_QUEEN  | b | (59 << FROM_SHIFT);
            field[60] = PIECE_KING   | b | (60 << FROM_SHIFT);
            field[61] = PIECE_BISHOP | b | (61 << FROM_SHIFT);
            field[62] = PIECE_KNIGHT | b | (62 << FROM_SHIFT);
            field[63] = PIECE_ROOK   | b | (63 << FROM_SHIFT);

            // Reihe 6 – schwarze Bauern
            for i in 48..56 {
                field[i] = PIECE_PAWN | b | ((i as u32) << FROM_SHIFT);
            }

            // Test Piece
            //field[40] = PIECE_PAWN   | w | (48u32 << FROM_SHIFT) ;
            field[25] = PIECE_KNIGHT | b | (25 << FROM_SHIFT);


            player_color = COLOR_BLACK;
        }
    }

    // Return field and player_color since they are always initialized
    (field, player_color)
}

#[allow(dead_code)]
/// !TODO \
/// Generates FEN using a string
pub fn generate_fen_from_field(field: &Field, flag_data: u32) -> Fen{
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
pub fn generate_field_from_move(mut field: Field, chess_move: u32) -> Field {
    let from = chess_move & FROM_MASK;
    let to = chess_move & TO_MASK;

    // get shifted for index
    let from_shifted = (from >> FROM_SHIFT) as usize;
    let to_shifted = (to >> TO_SHIFT) as usize;

    //println!("Field old : {:?}", &field);
    //println!("{}: {:b}| {}: {:b}", &from_shifted, &field[from_shifted], &to_shifted, &field[to_shifted]);

    // get and remove old piece
    let piece_to_move = field[from_shifted];
    field[from_shifted] = PIECE_NONE;

    // place new piece
    field[to_shifted] = piece_to_move & !FROM_MASK | to << FROM_TO_SHIFT;

    //println!("{}: {:b}| {}: {:b}", &from_shifted, &field[from_shifted], &to_shifted, &field[to_shifted]);
    //println!("Field new : {:?}", &field);
    field
}


/// TODO! \
/// Find all legal chess_moves a piece can make on the board
fn find_legal_moves(piece: Piece, field: &Field, flag_data: FlagData, player_color: u32) -> Vec<ChessMove> {
    let mut legal_moves: Vec<ChessMove> = Vec::new();

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
    let en_passant_square = (flag_data & EN_PASSANT_MASK) >> EN_PASSANT_SHIFT;

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

                    // En Passant
                    if position + 7 == en_passant_square {
                        legal_moves.push(position + 7);
                    }
                    if  position + 9 == en_passant_square {
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
                let left_knight_squares: [u32; 16]  = [7,15,23,31,39,47,55,63,6,14,22,30,38,46,54,62];
                let right_knight_squares: [u32; 16] = [0,8,16,24,32,40,48,56,1,9,17,25,33,41,49,57];
                let left_squares: [u32; 8]          = [7,15,23,31,39,47,55,63];
                let right_squares: [u32; 8]         = [0,8,16,24,32,40,48,56];

                // 2 vertical 1 horizontal
                // 2 up 1 left
                if  position < 48 && !left_squares.contains(&position) && (field[(position+17) as usize] & COLOR_MASK != player_color || field[(position+17) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position+17);
                }
                // 2 down 1 left
                if position > 15 && !left_squares.contains(&position) && (field[(position-15) as usize] & COLOR_MASK != player_color || field[(position-15) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position-15);
                }

                // 2 up 1 right
                if position < 48 && !right_squares.contains(&position) && (field[(position+15) as usize] & COLOR_MASK != player_color || field[(position+15) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position+15);
                }
                // 2 down 1 right
                if position > 15 && !right_squares.contains(&position) && (field[(position-17) as usize] & COLOR_MASK != player_color || field[(position-17) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position-17);
                }


                // 1 vertical 2 horizontal
                // 1 up 2 left
                if position < 56 && !left_knight_squares.contains(&position) && (field[(position+10) as usize] & COLOR_MASK != player_color || field[(position+10) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position+10);
                }
                // 1 down 2 left
                if position > 7 && !left_knight_squares.contains(&position) && (field[(position-6) as usize] & COLOR_MASK != player_color || field[(position-6) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position-6);
                }
                // 1 up 2 right
                if position < 56 && !right_knight_squares.contains(&position) && (field[(position+6) as usize] & COLOR_MASK != player_color || field[(position+6) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position+6);
                }
                // 1 up 2 right
                if position > 7 && !right_knight_squares.contains(&position) && (field[(position-10) as usize] & COLOR_MASK != player_color || field[(position-10) as usize] & PIECE_MASK == PIECE_NONE){
                    legal_moves.push(position-10);
                }
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

/// TODO! \
/// Evaluates a single position and returns it's evaluated score
pub fn evaluate_single_position(field: &Field) -> EvaluationScore {
    let piece_value = move |piece: &Piece| -> EvaluationScore{
        let piece = piece & PIECE_MASK;
        match piece {
            PIECE_PAWN => 1,
            PIECE_KNIGHT => 3,
            PIECE_BISHOP => 3,
            PIECE_ROOK => 5,
            PIECE_ROOK_MOVED => 5,
            PIECE_QUEEN => 9,

            // Has no value -> always there
            PIECE_KING => 0,

            PIECE_NONE | _  => 0
        }
    };

    let mut white_value: EvaluationScore = 0;
    let mut black_value: EvaluationScore = 0;

    for piece in field{
        match piece & COLOR_MASK{
            COLOR_WHITE => white_value += piece_value(&piece),
            COLOR_BLACK | _ => black_value += piece_value(&piece),
        }
    }

    // return
    white_value-black_value
}

// Production Accessible functions (Other are available/public due to testing reasons)
/// TODO! \
/// Finds the best possible move according to the evaluation sore from `evaluate_single_position()`
pub fn find_best_move(fen: Option<&Fen>) -> (ChessMove, EvaluationScore){
    // Generate Field from FEN
    let (field , player_color): (Field, u32) = generate_field_from_fen(fen);

    let flag_data: FlagData = 0b000001_000000_00000000000000000000; // [6] moves_since_pawn, [6] en_passant_square, [20] unused
    let mut legal_moves: Vec<Vec<u32>> = Vec::with_capacity(64);

    for i in 0..64 {
        let chess_move = field[i];
        legal_moves.push(find_legal_moves(chess_move, &field, flag_data, player_color));
    }

    println!("{:?}", legal_moves);


    // ########################
    // ## Evaluation Tactics ##
    // ########################

    // ## Single Depth - No Alpha-Beta pruning ##
    // Most simple Szenario: no a-b pruning, checking every single move till the end (here firstly only one move)

    let mut current_best_eval: EvaluationScore;
    let mut current_best_move: ChessMove = 0;
    //println!("INIT best_move: {:b} with eval: {}", &current_best_move, &current_best_eval);

    match player_color{
        COLOR_WHITE => {
            for (piece_index, piece_moves) in legal_moves.iter().enumerate(){
                for chess_move in piece_moves{
                    let field = generate_field_from_move(field, *chess_move);
                    let eval = evaluate_single_position(&field);
                    if eval > current_best_eval {
                        current_best_eval = eval;
                        current_best_move = (chess_move << TO_SHIFT) | (piece_index as u32) << FROM_SHIFT;
                        println!("best_move: {:b} with eval: {}", &current_best_move, &current_best_eval);
                    }
                }
            }
        }
        COLOR_BLACK => {
            for (piece_index, piece_moves) in legal_moves.iter().enumerate(){
                for chess_move in piece_moves{
                    let eval = evaluate_single_position(&field, *chess_move);
                    if eval < current_best_eval {
                        current_best_eval = eval;
                        current_best_move = (chess_move << TO_SHIFT) | (piece_index as u32) << FROM_SHIFT;
                        println!("best_move: {:b} with eval: {}", &current_best_move, &current_best_eval);
                    }
                }
            }
        }
        _ => {}
    }



    // Return the best move
    (current_best_move, current_best_eval)
}