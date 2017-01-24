fn main() {
    let mut v = vec![1, 2, 3];

    for i in &v {
        println!("reference {}", i);
    }

    for i in &mut v {
        println!("mut reference {}", i);
    }

    for i in v {
        println!("value {}", i);
    }

}
