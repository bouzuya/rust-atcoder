use proconio::input;

fn main() {
    input! {
        mut a: i64,
        mut b: i64,
        k: usize,
    };
    for i in 0..k {
        if i % 2 == 0 {
            if a % 2 != 0 {
                a -= 1;
            }
            a /= 2;
            b += a;
        } else {
            if b % 2 != 0 {
                b -= 1;
            }
            b /= 2;
            a += b;
        }
    }
    println!("{} {}", a, b);
}
