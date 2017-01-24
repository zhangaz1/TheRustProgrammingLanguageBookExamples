fn main() {
    println!("1 add one {}", add_one(1));

    let x = diverges();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverges() -> ! {
    panic!("This function never returns!");
}
