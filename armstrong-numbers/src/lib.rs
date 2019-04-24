pub fn is_armstrong_number(num: u32) -> bool {
    // unimplemented!("true if {} is an armstrong number", num)
    let digit = digit(num);
    let len = digit.len() as u32;
    let mut n = 0;

    for i in digit {
        n += i.pow(len);
    }
    n == num
}  
//non armstrong number
fn digit(mut num: u32) -> Vec<u32> {
    let mut digit = vec![];  
      
    while num != 0 {
        digit.push(num % 10);
        num /= 10;
    }

    digit
}