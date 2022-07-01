use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize,
    };
    let ans = (a..=b).any(|x| x % k == 0);
    println!("{}", if ans { "OK" } else { "NG" });
}
