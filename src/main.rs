fn main() {
    let v = vec![1, 2, 3];
    let v = takev(v);
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

    let b = true;
    let notB = toggle(b);
    println!("b = {}, notB = {}", b, notB);

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let (v1, v2, answer) = return_all(v1, v2);
    println!("v1 = {}, v2 = {}, answer = {}", v1[0], v2[0], answer);

}

fn return_all(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    (v1, v2, 5)
}

fn toggle(b: bool) -> bool {
    !b
}

fn double(n: i32) -> i32 {
    n * 2
}

fn take (n: i32) {

}

fn takev (v: Vec<i32>) -> Vec<i32> {
    v
}
