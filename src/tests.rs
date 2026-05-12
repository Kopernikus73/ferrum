#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_evaluation() {
        let fens: [String; 1] = [
            String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        ];

        for fen in fens.into_iter(){
            let evaluation = evaluation_engine::find_best_move(Some(&fen));
            println!("\x1b[32mEvaluation:\x1b[0m{}", evaluation.0);
        }
    }

    #[test]
    fn test_fen_generation(){
        let fields: [[u32; 64]; 1] = [
            [0; 64]
        ];
        let flag_data: [u32; 1] = [
            0
        ];

        for (i, field) in fields.into_iter().enumerate(){
            let fen = evaluation_engine::generate_fen(&field, flag_data[i]);
            println!("\x1b[32mFEN:\x1b[0m{}", fen);
        }
    }

    #[test]
    fn test_field_generation(){
        let fens: [String; 1] = [String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")];

        for fen in fens.into_iter(){
            let field = evaluation_engine::generate_field(Some(&fen));
            println!("\x1b[32mField:\x1b[0m{:?}", field);
        }
    }


}