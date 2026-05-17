use crate::evaluation_engine::{find_best_move, FROM_MASK, FROM_SHIFT, TO_MASK, TO_SHIFT};

mod evaluation_engine;

// Tests import
#[cfg(test)]
mod tests;

fn main(){
    // Start timer
    let start_time = std::time::Instant::now();

    // TEST Get position value
    //let eval = evaluation_engine::evaluate_single_position(&generate_field_from_fen(Some(&String::from("1"))).0, 0b0_000_001000_010000_0_00_0000000000000);
    //println!("{}", eval);

    // Test find_best_move function
    let best_move = find_best_move(Some(&String::from("1")));
    println!("from->to: {}->{} with eval: {}", (best_move.0 & FROM_MASK) >> FROM_SHIFT, (best_move.0 & TO_MASK) >> TO_SHIFT, best_move.1);

    // Get elapsed time since the start
    let elapsed_time = start_time.elapsed().as_nanos();
    println!("------------------------------------");
    println!("\x1b[33mElapsed time:\x1b[0m {}", format_duration(elapsed_time));
    println!("------------------------------------");
}


// Debugging and Performance Purposes
fn format_duration(nanos: u128) -> String {
    const NANOS_PER_MICRO: u128 = 1_000;
    const NANOS_PER_MILLI: u128 = 1_000_000;
    const NANOS_PER_SECOND: u128 = 1_000_000_000;

    if nanos < NANOS_PER_MICRO {
        format!("{:.2} ns", nanos as f64)
    } else if nanos < NANOS_PER_MILLI {
        format!("{:.2} µs", nanos as f64 / NANOS_PER_MICRO as f64)
    } else if nanos < NANOS_PER_SECOND {
        format!("{:.2} ms", nanos as f64 / NANOS_PER_MILLI as f64)
    } else {
        format!("{:.2} s", nanos as f64 / NANOS_PER_SECOND as f64)
    }
}


// ERROR CODES
//              10 - 49 (user error)
//              50 - 99 (internal error)
// 10 -> No fen given
// 50 -> internal error: could not find char