fn main() {
    let num : u64 = 5;
    let result : u64 = (1..=num).collect::<Vec<u64>>().iter().fold(1, |acc, n| acc * n);

    println!("{:#?}", result);
}
