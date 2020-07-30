use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        b: [i64; n],
        c: [i64; n - 1],
    };
    let mut ans = 0_i64;
    let mut p = None;
    for &a_i in a.iter() {
        ans += b[a_i];
        if let Some(i_p) = p {
            if a_i == i_p + 1 {
                ans += c[i_p];
            }
        }
        p = Some(a_i);
    }
    println!("{}", ans);
}
