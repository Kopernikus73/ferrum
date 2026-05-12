mod evaluation_engine;

// Tests import
#[cfg(test)]
mod tests;

fn main(){
    // Start timer
    let start_time = std::time::Instant::now();

    // Get the best move / position
    let best_move = evaluation_engine::find_best_move(Some(&String::from("1")));
    let _best_field = evaluation_engine::generate_fen(&evaluation_engine::generate_field_from_move(best_move.0), best_move.1);

    // Get elapsed time since the start
    let elapsed_time = start_time.elapsed().as_nanos();
    println!("------------------------------------");
    println!("Elapsed time: {}", format_duration(elapsed_time));
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