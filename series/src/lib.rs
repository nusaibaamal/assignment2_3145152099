pub fn series(digits: &str, len: usize) -> Vec<String> {
    // unimplemented!(
    //     "What are the series of length {} in string {:?}",
    //     len,
    //     digits
    // )
    (len..=digits.len()).map(|x| digits[x-len..x].to_string()).collect()
}