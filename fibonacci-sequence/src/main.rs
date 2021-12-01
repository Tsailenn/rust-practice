fn main() {
    let num = 9;

    let mut res : Vec<u32> = (0..num).collect::<Vec<u32>>();
    res[0] = 1; res[1] = 1;

    for i in 1..(res.len()-1) {
        res[(i+1) as usize] = res[i as usize] + res[i-1 as usize];
    }

    println!("{:#?}", res);
}
