fn main() {
    let inp = "among uus";
    let vowels : [char ; 5] = ['a', 'i', 'u', 'e', 'o'];

    let count : u16 = inp.chars().fold(0, |mut acc, x| -> u16 {
        if vowels.contains(&x) {
            acc += 1;
        }
        acc
    });

    println!("{}", count);
}
