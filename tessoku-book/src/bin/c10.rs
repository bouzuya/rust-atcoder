use proconio::input;

fn main() {
    input! {
        w: usize,
    };

    // 12 * 7^(w-1)
    let p = 1_000_000_007_usize;
    let mut b = 7_usize;
    let mut x = w - 1;
    let mut y = 12;
    for _ in 0..60 {
        if (x & 1) == 1 {
            y *= b;
            y %= p;
        }
        b *= b;
        b %= p;
        x >>= 1;
    }

    let ans = y;
    println!("{}", ans);
}
