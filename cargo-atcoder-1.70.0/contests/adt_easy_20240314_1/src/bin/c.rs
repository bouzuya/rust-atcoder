use proconio::input;

fn main() {
    input! {
        k: usize,
        g: usize,
        m: usize,
    };
    let mut glass = 0_usize;
    let mut mug = 0_usize;
    for _ in 0..k {
        if glass == g {
            glass = 0;
        } else if mug == 0 {
            mug = m;
        } else {
            let d = (g - glass).min(mug);
            mug -= d;
            glass += d;
        }
    }
    println!("{} {}", glass, mug);
}
