#[cfg(test)]
mod tests {
    const NUMBER_TESTS_GENERATE_FIELD_FROM_FEN: usize = 1;
    const NUMBER_TESTS_GENERATE_FEN_FROM_FIELD: usize = 1;

    use crate::evaluation_engine::*;
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

            tests_passed += if field.0 == expected_fields[i]{
                1
            } else {
                0
            };
        }

        assert_eq!(NUMBER_TESTS_GENERATE_FIELD_FROM_FEN, tests_passed);
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
            println!("\x1b[32mMove:\x1b[0m{}", evaluation.0);
            println!("\x1b[32mEvaluation:\x1b[0m{}", evaluation.1);
        }
    }
}