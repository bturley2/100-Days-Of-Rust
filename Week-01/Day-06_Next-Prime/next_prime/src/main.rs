fn main() {
    println!("Hello, world!");

    //next_prime(12) ➞ 13
    println!("12 -> {}", next_prime(12));
    println!("24 -> {}", next_prime(24));
    println!("11 -> {}", next_prime(11));
}

fn next_prime(x: i32) -> i32 {
    let mut cur = x;
    loop {
        if is_prime(cur) {
            return x;
        }
        cur += 1;
    }
}

fn is_prime(x: i32) -> bool {
    let check = (x as f64).sqrt() as i32;
    for i in 0..check {
        if x % i == 0 {
            return true;
        }
    }
    false
}
