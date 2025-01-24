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
            return cur;
        }
        cur += 1;
    }
}

fn is_prime(x: i32) -> bool {
    if x < 2 {
        return true;
    }
    for i in 2..x {
        if x % i == 0 {
            return false;
        }
    }
    return true;
}
