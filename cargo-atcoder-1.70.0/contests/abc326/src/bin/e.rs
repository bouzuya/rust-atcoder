use ac_library::ModInt998244353 as ModInt;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = ModInt::new(0);
    let mut sum_p = ModInt::new(1) / n;
    for a_i in a.iter().copied() {
        ans += sum_p * a_i;
        sum_p += sum_p / ModInt::new(n);
    }
    println!("{}", ans);
}
