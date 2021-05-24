use proconio::input;

fn main() {
    input! {
        k: usize,
    };
    let mut x = 7_usize;
    for i in 0..=1_000_000 {
        if x % k == 0 {
            println!("{}", i + 1);
            return;
        }
        x *= 10;
        x %= k;
        x += 7;
        x %= k;
    }
    println!("-1");
}
