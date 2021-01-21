use proconio::input;

fn main() {
    input! {
        se: [(i64, i64); 3],
    };
    let ans = se.iter().map(|(s_i, e_i)| s_i / 10 * e_i).sum::<i64>();
    println!("{}", ans);
}
