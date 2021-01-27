use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
    };
    if h > n || w > n {
        println!("0");
        return;
    }
    let ans = (n + 1 - h) * (n + 1 - w);
    println!("{}", ans);
}
