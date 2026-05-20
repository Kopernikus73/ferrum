// tests.rs
//
// Tests single functionalities

#[cfg(test)]
mod tests {
    use ferrum::{
        *,
        board::manipulation::*,
        constants::*,
        protocol::interaction::find_best_move
    };


    const NUMBER_TESTS_GENERATE_FIELD_FROM_FEN: usize = 1;
    const NUMBER_TESTS_GENERATE_FEN_FROM_FIELD: usize = 1;

    // ###########
    // ## Tests ##
    // ###########

    #[test]
    fn test_generate_field_from_fen(){
        let fens: [Fen; NUMBER_TESTS_GENERATE_FIELD_FROM_FEN] = [
            String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        ];
        let expected_fields: [Field; NUMBER_TESTS_GENERATE_FIELD_FROM_FEN] = [
            [0u32; 64],
        ];
        let mut tests_passed: usize = 0;

        for (i, fen) in fens.into_iter().enumerate(){
            let field = generate_field_from_fen(Some(&fen));
            //println!("{:?}", &field);
            debug_field_binary(&field.0, "Generated Field (binary)");

            tests_passed += if field.0 == expected_fields[i]{
                1
            } else {
                0
            };
        }

        assert_eq!(NUMBER_TESTS_GENERATE_FIELD_FROM_FEN, tests_passed);

        // Hilfsfunktion für die Testausgabe
        fn debug_field_binary(field: &Field, label: &str) {
            println!("{}:", label);
            for i in 0..64 {
                if i % 8 == 0 && i > 0 {
                    print!("\n ");
                }
                if i % 8 == 0 {
                    print!(" ");
                }
                print!("{:04b} ", (field[i] >> 28) & 0b1111);
            }
            println!("\n");
        }
    }
    
    #[test]
    fn test_generate_fen_from_field(){
        let fields: [Field; NUMBER_TESTS_GENERATE_FEN_FROM_FIELD] = [
            [
                PIECE_ROOK | COLOR_WHITE,
                PIECE_KNIGHT | COLOR_WHITE,
                PIECE_BISHOP | COLOR_WHITE,
                PIECE_QUEEN | COLOR_WHITE,
                PIECE_KING | COLOR_WHITE,
                PIECE_BISHOP | COLOR_WHITE,
                PIECE_KNIGHT | COLOR_WHITE,
                PIECE_ROOK | COLOR_WHITE,
                PIECE_PAWN | COLOR_WHITE,
                PIECE_PAWN | COLOR_WHITE,
                PIECE_PAWN | COLOR_WHITE,
                PIECE_PAWN | COLOR_WHITE,
                PIECE_PAWN | COLOR_WHITE,
                PIECE_PAWN | COLOR_WHITE,
                PIECE_PAWN | COLOR_WHITE,
                PIECE_PAWN | COLOR_WHITE,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                PIECE_PAWN | COLOR_BLACK,
                PIECE_PAWN | COLOR_BLACK,
                PIECE_PAWN | COLOR_BLACK,
                PIECE_PAWN | COLOR_BLACK,
                PIECE_PAWN | COLOR_BLACK,
                PIECE_PAWN | COLOR_BLACK,
                PIECE_PAWN | COLOR_BLACK,
                PIECE_PAWN | COLOR_BLACK,
                PIECE_ROOK | COLOR_BLACK,
                PIECE_KNIGHT | COLOR_BLACK,
                PIECE_BISHOP | COLOR_BLACK,
                PIECE_QUEEN | COLOR_BLACK,
                PIECE_KING | COLOR_BLACK,
                PIECE_BISHOP | COLOR_BLACK,
                PIECE_KNIGHT | COLOR_BLACK,
                PIECE_ROOK | COLOR_BLACK,
            ]
        ];
        let flag_data: [FlagData; NUMBER_TESTS_GENERATE_FEN_FROM_FIELD] = [
            0
        ];
        let expected_fens: [Fen; NUMBER_TESTS_GENERATE_FEN_FROM_FIELD] = [
            Fen::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        ];
        let mut tests_passed: usize = 0;

        for (i, field) in fields.into_iter().enumerate(){
            let fen = generate_fen_from_field(&field, flag_data[i]);

            tests_passed += if fen == expected_fens[i]{
                1
            } else {
                0
            };
        }
        assert_eq!(NUMBER_TESTS_GENERATE_FEN_FROM_FIELD, tests_passed);
    }


    // ################
    // ## Semi Tests ##
    // ################

    // Shows how the evaluation engine react to different FEN strings
    #[test]
    fn test_evaluate_single_position() {
        let fens: [Fen; 1] = [
            String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        ];

        for fen in fens.into_iter(){
            let evaluation = find_best_move(Some(&fen));
            //println!("\x1b[32mMove:\x1b[0m{}", evaluation.0);
            //println!("\x1b[32mEvaluation:\x1b[0m{}", evaluation.1);
        }
    }
}