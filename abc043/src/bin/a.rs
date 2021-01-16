use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = (1..=n).sum::<usize>();
    println!("{}", ans);
}
