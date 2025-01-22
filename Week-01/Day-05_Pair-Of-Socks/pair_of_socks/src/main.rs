use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    println!("'AA' -> {}", sock_pairs("AA")); // ➞ 1
    println!("'ABABC' -> {}", sock_pairs("ABABC")); // ➞ 2
    println!("'CABBACCC' -> {}", sock_pairs("CABBACCC")); // ➞ 4
}

fn sock_pairs(socks: &str) -> i32 {
    let mut pairs = 0;

    let mut count_map = HashMap::new();

    // create a map of all sock counts
    // this approach uses pattern matching, and requires semicolon to not return match value
    for sock in socks.bytes() {
        match count_map.get(&sock) {
            Some(v) => count_map.insert(sock, v + 1),
            None => count_map.insert(sock, 1),
        };
    }

    // this approach also uses pattern matching, but does not require semicolon since no value is
    // returned from each branch.
    //for sock in socks.bytes() {
    //    match count_map.get(&sock) {
    //        Some(v) => {
    //            count_map.insert(sock, v + 1);
    //        }
    //        None => {
    //            count_map.insert(sock, 1);
    //        }
    //    }
    //}

    //// create a map of all sock counts
    //// this approach checks for single value
    //for sock in socks.bytes() {
    //    if let Some(count) = count_map.get(&sock) {
    //        count_map.insert(sock, count + 1);
    //    } else {
    //        count_map.insert(sock, 1);
    //    }
    //}

    // iterate through map and count pairs
    for (_, v) in count_map.iter() {
        pairs += v / 2;
    }

    pairs
}
