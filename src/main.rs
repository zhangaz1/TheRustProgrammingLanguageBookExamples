fn main() {
    let f: fn(i32) -> i32 = add_one;
    println!("1 add one {}", f(1));

    // let x = diverges();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// fn diverges() -> ! {
//     panic!("This function never returns!");
// }
