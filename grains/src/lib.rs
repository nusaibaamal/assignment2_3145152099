pub fn square(s: u32) -> u64 {
    // unimplemented!("grains of rice on square {}", s);

    //https://en.wikipedia.org/wiki/Wheat_and_chessboard_problem
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    u64::pow(2, s-1) //https://doc.rust-lang.org/nightly/std/?search=pow
}

pub fn total() -> u64 {
    (1..=64).map(|x|square(x)).sum(); //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map

}
