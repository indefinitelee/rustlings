// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.


fn main() {
    // mut for answer 3 to work
    let mut vec0 = Vec::new();
//three ways
//1 clone vec0
//    let mut vec1 = fill_vec(vec0.clone());
//2 fill vec borrows
    // let mut vec1 = fill_vec(&vec0);
//3 new function 
let mut vec1 = fill_vec_with_mutable_reference(&mut vec0);


    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

//answer 3 mutably borrow
fn fill_vec_with_mutable_reference(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.clone()
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
