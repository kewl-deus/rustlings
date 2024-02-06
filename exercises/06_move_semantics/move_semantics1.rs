// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    //println!("{}", join_comma(&vec0));
    //let _x = vec0[0];

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn join_comma(vec: &Vec<i32>) -> String {
    println!("hi");
    join(vec, ", ")
}

fn join(vec: &Vec<i32>, sep: &str) -> String {
    vec.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(sep)
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    //ownership move here!!!
    let mut vec = vec;

    vec.push(88);

    vec
}
