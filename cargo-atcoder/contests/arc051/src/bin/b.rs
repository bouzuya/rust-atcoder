use proconio::input;

fn main() {
    input! {
        k: i64
    };
    let mut a = 1;
    let mut b = 1;
    for _ in 0..k - 1 {
        let c = a + b;
        a = b;
        b = c;
    }
    println!("{} {}", a, b);
}
