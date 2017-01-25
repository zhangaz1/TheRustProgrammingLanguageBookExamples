fn main() {
    let v = vec![1, 2, 3];
    // takev(v);
    println!("{}", v[1]);

    let x = 5;
    take(x);
    println!("{}", x);

    let i = 1;
    let i2 = i;
    println!("i = {}", i);

    let a = 5;
    let y = double(a);
    println!("a = {}, y = {}", a, y);
}

fn double(n: i32) -> i32 {
    n * 2
}

fn take (n: i32) {

}

fn takev (v: Vec<i32>) {

}
