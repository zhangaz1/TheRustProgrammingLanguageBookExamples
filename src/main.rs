fn main() {
    let v = vec![1, 2, 3];
    {
        let v2 = v;
    }
    println!("{}", v[1]);

    let x = 5;
    {
        let x2 = &x;
    }
    println!("{}", x);
}
