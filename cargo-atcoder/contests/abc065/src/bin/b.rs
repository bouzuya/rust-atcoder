use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let mut count = 0_i64;
    let mut a_c = 0;
    for _ in 0..=n {
        count += 1;
        a_c = a[a_c];
        if a_c == 1 {
            break;
        }
    }
    let ans = if count == (n + 1) as i64 { -1 } else { count };
    println!("{}", ans);
}
