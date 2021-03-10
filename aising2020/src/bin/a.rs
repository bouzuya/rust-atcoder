use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
        d: usize,
    };
    let ans = (l..=r).filter(|&x| x % d == 0).count();
    println!("{}", ans);
}
