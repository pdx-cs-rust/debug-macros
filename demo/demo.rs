use debug_macros::debug;

fn main() {
    debug!();
    let n: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut sum = 0;
    debug!("starting sum");
    for i in 0..n {
        debug!("round", i);
        sum += i;
    }
    println!("{}", sum);
    debug!("done");
}
