pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // unimplemented!(
    //     "Sum the multiples of all of {:?} which are less than {}",
    (1..limit).filter(|i| factors.iter().any(|&n| n != 0 && i % n == 0)).sum()
}