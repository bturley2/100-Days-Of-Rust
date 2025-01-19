fn main() {
    /*
    ["--xo--x--ox--",
    "--xx--x--xx--",
    "--oo--o--oo--",      <<< vegetarian skewer
    "--xx--x--ox--",
    "--xx--x--ox--"]
    */

    let in1: Vec<&str> = [
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--",
    ]
    .to_vec();

    let in2: Vec<&str> = [
        "--oooo-ooo--",
        "--xxxxxxxx--",
        "--o---",
        "-o-----o---x--",
        "--o---o-----",
    ]
    .to_vec();

    //let (veg_count, meat_count) = get_veg_meat_count(in1);
    //println!("in1 total [veg, meat] = [{},{}]", veg_count, meat_count);
    let (full_veggie, non_veggie) = count_veggie_skewers(in1);
    println!(
        "in1 has {} all-veggie skewers, and {} non-veggie skewers",
        full_veggie, non_veggie
    );

    //let (veg_count, meat_count) = get_veg_meat_count(in2);
    //println!("in2 total [veg, meat] = [{},{}]", veg_count, meat_count);
    let (full_veggie, non_veggie) = count_veggie_skewers(in2);
    println!(
        "in2 has {} all-veggie skewers, and {} non-veggie skewers",
        full_veggie, non_veggie
    );
}

fn count_veggie_skewers(skewers: Vec<&str>) -> (u32, u32) {
    let mut all_veg_count = 0;
    let mut non_veg_count = 0;
    for skewer in skewers {
        let mut is_meat_found = false;
        for c in skewer.chars() {
            if c == 'x' {
                is_meat_found = true;
                break;
            }
        }
        if is_meat_found {
            non_veg_count += 1
        } else {
            all_veg_count += 1
        }
    }

    return (all_veg_count, non_veg_count);
}

fn get_veg_meat_count(skewers: Vec<&str>) -> (u32, u32) {
    let mut veg_count = 0;
    let mut meat_count = 0;
    for skewer in skewers {
        for c in skewer.chars() {
            match c {
                'x' => meat_count += 1,
                'o' => veg_count += 1,
                _ => (),
            }
        }
    }
    return (veg_count, meat_count);
}
