fn main() {
    println!("Hello, world!");

    find_nemo("I am finding Nemo !");
    find_nemo("Nemo is me");
    find_nemo("I Nemo am");
    find_nemo("There was no fishy...")
}

/*
fn find_nemo2(text: &str) {
    let nemo = "Nemo";
    println!("Searching: '{text}'");

    // uses pattern matching and loops through iterator differently
    let mut split_pieces: Split<&str> = text.split(' ');

    loop {
        match split_pieces.next() {
            Some(nemo) => println!(""),
            Some(s) => println!(""),
            None => println!(""),
        }
    }
}
*/

fn find_nemo(text: &str) {
    let nemo = "Nemo";
    println!("Searching: '{text}'");

    // split apart the string into elements
    let pieces: Vec<&str> = text.split(' ').collect();

    let mut found: bool = false;

    for (idx, word) in pieces.iter().enumerate() {
        if *word == nemo {
            println!("Found {nemo} at {:?}", idx);
            found = true;
        }
    }

    if !found {
        let err_msg: &str = "I can't find Nemo :(";
        println!("{err_msg}");
    }
}
