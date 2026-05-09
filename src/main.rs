mod evaluation_engine;

fn main(){
    println!("eval: {}", evaluation_engine::eval(String::from("1")))
}

// ERROR CODES
// 3 -> No fen given