use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    a.sort_by_key(|&x| -x);
    a.dedup();
    let ans = a[1];
    println!("{}", ans);
}
