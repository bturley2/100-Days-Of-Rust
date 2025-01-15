fn main() {
    println!("Hello, world!");

    find_nemo("I am finding Nemo !");
    find_nemo("Nemo is me");
    find_nemo("I Nemo am");
    find_nemo("There was no fishy...")
}

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
