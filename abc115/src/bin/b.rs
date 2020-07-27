use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i64; n],
    };
    let sum = p.iter().sum::<i64>();
    let max = p.iter().max().unwrap();
    let ans = sum - max + max / 2;
    println!("{}", ans);
}
