pub fn nth(n: u32) -> u32 {
    // unimplemented!("What is the 0-indexed {}th prime number?", n)
    if n == 0 {
        return 2
    }
	
	let mut index = 0;
	let mut prime = 2;
	
pub fn primes(num: u32) -> bool {
	if num == 2 {
        return true
    }
	for x in 2..num {
		if num % x == 0 {
			return false
		}
	}
	return true
}

	while index != n {
		prime += 1;
		if primes(prime) == true {
			index += 1;	
		}
	}
	prime
}