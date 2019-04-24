pub fn raindrops(n: u32) -> String {
    //unimplemented!("what sound does Raindrop #{} make?", n)
    let mut rd = String::new();
    // let x= String::from("n");

    if n % 3 == 0 && n % 7 != 0 && n % 5 != 0 {
        rd.push_str("Pling")  
    }
    else if n % 5 == 0 && n % 3 !=0 && n % 7 != 0 {
        rd.push_str("Plang")
    }
    else if n % 7 == 0 && n % 3 != 0 && n % 5 != 0 {
        rd.push_str("Plong")
    }
    else if n % 3 == 0 && n % 5 == 0 && n % 7 != 0 {
        rd.push_str("PlingPlang")
    }
    else if n % 3 == 0 && n % 7 == 0 && n % 5 != 0 {
    rd.push_str("PlingPlong")
    }
    else if n % 7 == 0 && n % 5 == 0 && n % 3 != 0 {
        rd.push_str("PlangPlong")   
    }
    else if n % 3 == 0 && n % 5 == 0 && n % 7 == 0 {
        rd.push_str("PlingPlangPlong")
    }
    // else {
    //     rd.format!("{}", n)
    // }
    else {
        rd.push_str(&n.to_string())
    }
    return rd;
}