fn main() {
    let week1: Vec<i32> = [3, 4, 1, 2].to_vec();
    println!("Longest Stretch was: {}", longest_progress(week1));

    let week2: Vec<i32> = [10, 11, 12, 9, 10].to_vec();
    println!("Longest Stretch was: {}", longest_progress(week2));

    let week3: Vec<i32> = [6, 5, 4, 3, 2, 9].to_vec();
    println!("Longest Stretch was: {}", longest_progress(week3));

    let week4: Vec<i32> = [9, 9].to_vec();
    println!("Longest Stretch was: {}", longest_progress(week4));
}

// this algorithm isn't quite what the prompt asked for, this finds longest consecutive increase,
// not count the number of increases from week to week, but it served its learning purpose for
// today.
fn longest_progress(days: Vec<i32>) -> i32 {
    let mut longest = if days.is_empty() { 0 } else { 1 };
    let mut cur = 0;
    let mut last = -1;
    for x in days.iter() {
        if *x > last {
            // update count && potentially 'longest'
            last = *x as i32;
            cur += 1;
            if cur > longest {
                longest += 1;
            }
        } else {
            // reset count
            cur = 0;
        }
    }

    return longest;
}
