use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ss: [String; n],
    };
    ss.sort();
    ss.dedup();
    let ans = ss.len();
    println!("{}", ans);
}
