fn main() {
    let amogus : &str  = "among us";

    let res : String = amogus.chars().rev().collect::<String>();
    println!("{}", res);
}
