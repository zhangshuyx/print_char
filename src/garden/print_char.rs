pub fn print_char_Az() {
    for i in 'A'..='z' {
        println!("{i}");
    }
}

pub fn print_char_aZ() {
    for i in ('Z'..='a').rev() {
        println!("{i}");
    }
}