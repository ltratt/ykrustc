use std::env;

#[no_sw_trace]
fn main() {
    let x = env::args().count();
    let mut res = 42;

    if x > 4 {
        res = 100;
    }

    println!("res: {}", res);
}

// END RUST SOURCE
// [Begin SIR for main]
// ...
// $2: t0 = I32(42)
// ...
// [End SIR for main]
