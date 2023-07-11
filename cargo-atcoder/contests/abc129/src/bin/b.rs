use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [i64; n],
    };
    let sum = w.iter().sum::<i64>();
    let mut s_1 = 0_i64;
    let mut ans = sum;
    for w_i in w {
        s_1 += w_i;
        ans = ans.min((s_1 - (sum - s_1)).abs());
    }
    println!("{}", ans);
}
