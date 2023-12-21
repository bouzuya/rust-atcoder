use proconio::input;

fn main() {
    input! {
        capital_k: usize,
        capital_g: usize,
        capital_m: usize,
    };
    let mut g = 0_usize;
    let mut m = 0_usize;
    for _ in 0..capital_k {
        if g == capital_g {
            g = 0;
        } else if m == 0 {
            m = capital_m;
        } else {
            while !(m == 0 || g == capital_g) {
                if capital_g > g {
                    let d1 = capital_g - g;
                    let d2 = m;
                    if d1 <= d2 {
                        g += d1;
                        m -= d1;
                    } else {
                        g += d2;
                        m -= d2;
                    }
                }
            }
        }
    }
    println!("{} {}", g, m);
}
