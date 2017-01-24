fn main() {
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            println!("{},{}", x, y);

            if x % 2 == 0 {
                continue 'outer;
            }

            if y % 2 == 0 {
                continue 'inner;
            }

            println!("x: {}, y: {}", x, y);
        }
    }
}
