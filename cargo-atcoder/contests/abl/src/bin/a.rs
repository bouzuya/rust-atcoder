use proconio::input;

fn main() {
    input! {
        k: usize,
    };
    let ans = "ACL".repeat(k);
    println!("{}", ans);
}
