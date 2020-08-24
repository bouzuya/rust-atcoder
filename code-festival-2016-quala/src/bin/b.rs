use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let mut ans = 0;
    for (i, &j) in a.iter().enumerate() {
        if i == a[j] && i < j {
            ans += 1;
        }
    }
    println!("{}", ans);
}
