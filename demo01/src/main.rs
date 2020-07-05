fn main() {
    string_slice()
}

fn string_slice(){
    let s = String::from("pain is me");
    let s1 = &s[0..4];
    let s2 = &s[0..=3];
    let s3 = &s[..=3];

    println!("s1 is {}",s1);
    println!("s2 is {}",s2);
    println!("s3 is {}",s3);
}