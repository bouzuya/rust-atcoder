use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t: [i64; n],
    };
    t.sort();
    let ans = t[0];
    println!("{}", ans);
}
