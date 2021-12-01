fn main() {
    let n = 13;

    let mut primes : Vec<u64> = vec![2];

    for i in 2..(n+1) {

        let mut count_down = primes.len();
        for j in primes.iter() {
            if i % j == 0 { //divisible
                break;
            }
            else {
                count_down -= 1;
            }
        }
        if count_down == 0 {
            primes.push(i);
        }
    }
    
    println!("{:#?}", primes[primes.len() - 1]);
}
