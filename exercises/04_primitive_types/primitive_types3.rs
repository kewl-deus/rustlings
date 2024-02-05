// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let a = [42; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
        let joined = a.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
        println!("Content: {}", joined);
        /*
        for i in 0..a.len() {
            println!("{i} : {}", a[i]);
        }
        */

    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
