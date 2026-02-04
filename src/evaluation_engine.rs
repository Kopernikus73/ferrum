use phf::phf_map;

// Masks
const COLOR_MASK: u32 =
    0b1_000_000000_000000_0_00_0000000000000;

const PIECE_MASK: u32 =
    0b0_111_000000_000000_0_00_0000000000000;

const FROM_MASK: u32 =
    0b0_000_111111_000000_0_00_0000000000000;

const TO_MASK: u32 =
    0b0_000_000000_111111_0_00_0000000000000;

const PROMOTE_MASK: u32 =
    0b0_000_000000_000000_1_00_0000000000000;

const CHECK_MASK: u32 =
    0b0_000_000000_000000_0_11_0000000000000;

// Pieces
pub const PIECE_NONE:   u32 = 0b0_000_000000_000000_0_00_00000000000;
pub const PIECE_PAWN:   u32 = 0b0_001_000000_000000_0_00_00000000000;
pub const PIECE_KNIGHT: u32 = 0b0_010_000000_000000_0_00_00000000000;
pub const PIECE_BISHOP: u32 = 0b0_011_000000_000000_0_00_00000000000;
pub const PIECE_ROOK:   u32 = 0b0_100_000000_000000_0_00_00000000000;
pub const PIECE_ROOK_MOVED:   u32 = 0b0_101_000000_000000_0_00_00000000000;
pub const PIECE_QUEEN:  u32 = 0b0_110_000000_000000_0_00_00000000000;
pub const PIECE_KING:   u32 = 0b0_111_000000_000000_0_00_00000000000;

// Colors
pub const COLOR_WHITE: u32 = 0b0_000_000000_000000_0_00_00000000000;
pub const COLOR_BLACK: u32 = 0b1_000_000000_000000_0_00_00000000000;


fn generate_field(fen: Option<String>) -> Vec<u32> {
    let mut field = vec![0; 64];

    match fen{
        None => {
            vec![]
        }
        _ => {// === Weiße Figuren ===
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
                field[i] = PIECE_PAWN | w;
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
                field[i] = PIECE_PAWN | b;
            }

            field}
    }
}

pub fn eval(chess_move: u32, field: Vec<u32>) -> u32{
    let field: Vec<u32> = if field.is_empty(){
        generate_field(None)
    } else {
        field
    };




    println!("{:?}", field);

    10
}

fn find_legal_moves(piece: u32, field: &Vec<u32>, ) -> Vec<u32> {
    let mut legal_moves = Vec::new();

    let color = piece & COLOR_MASK;
    let piece_type = piece & PIECE_MASK;
    let position = (piece & FROM_MASK) >> 22; // 22 → position wird auf die ersten 6 bits gemapped

    match piece_type {
        PIECE_NONE => {}
        PIECE_PAWN => {
            // Kein over-/underflow check nötig → kann nicht am Rand stehen
            if color == COLOR_WHITE {
                // move forward

                if field[(position + 8) as usize] == 0{
                    legal_moves.push(position + 8);
                }
                if field[(position + 16) as usize] == 0 && position < 16 && position > 7{
                    legal_moves.push(position + 16);
                }

                // capture

                
            } else {
                // move forward
                if field[(position - 8) as usize] == 0{
                    legal_moves.push(position - 8);
                }
                if field[(position - 16) as usize] == 0 && position < 56 && position > 40{
                    legal_moves.push(position - 16);
                }

                // capture
            }
        }
        PIECE_KNIGHT => {
            // Knight moves
        }
        PIECE_BISHOP => {
            // Bishop moves
        }
        PIECE_ROOK | PIECE_ROOK_MOVED => {
            // Rook moves
        }
        PIECE_QUEEN => {
            // Queen moves
        }
        PIECE_KING => {
            // King moves
        }
        _ => {
            // unreachable (Als Sicherheit)
        }
    }

    legal_moves
}
