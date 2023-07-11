use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let sum = a.iter().sum::<i64>();
    let avg1 = sum / n as i64;
    let avg2 = avg1 + 1;
    let mut ans1 = 0;
    let mut ans2 = 0;
    for a_i in a {
        ans1 += (avg1 - a_i).pow(2);
        ans2 += (avg2 - a_i).pow(2);
    }
    let ans = std::cmp::min(ans1, ans2);
    println!("{}", ans);
}
