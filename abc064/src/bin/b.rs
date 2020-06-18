use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let ans = a.iter().max().unwrap() - a.iter().min().unwrap();
    println!("{}", ans);
}
