pub fn factors(n: u64) -> Vec<u64> {
    //unimplemented!("This should calculate the prime factors of {}", n)
    let mut factors = Vec::new();
    let mut pembagi = 2;
    let mut number = n;

    while pembagi <= number {
        if number % pembagi == 0 {
            number /= pembagi;
            factors.push(pembagi);
        } else {
            pembagi += 1;
        }
    }
    factors
}