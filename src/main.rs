fn main() {
    let index: usize = 2;
    let v = vec![1, 2, 3];

    match v.get(index) {
        Some(x) => println!("Item {} is {}", index, x),
        None => println!("Sorry, this vector is too short."),
    }
}
