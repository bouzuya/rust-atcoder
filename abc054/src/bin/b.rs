use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; n],
        b: [Chars; m],
    };
    let mut ans = false;
    for y_t in 0..n - m + 1 {
        for x_l in 0..n - m + 1 {
            let mut eq = true;
            for y in 0..m {
                for x in 0..m {
                    if a[y_t + y][x_l + x] != b[y][x] {
                        eq = false;
                        break;
                    }
                }
            }
            if eq {
                ans = eq;
                break;
            }
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
