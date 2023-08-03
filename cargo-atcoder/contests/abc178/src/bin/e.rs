use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let z = xy.iter().map(|&(x, y)| x + y).collect::<Vec<i64>>();
    let w = xy.iter().map(|&(x, y)| x - y).collect::<Vec<i64>>();
    let ans = (z.iter().max().unwrap() - z.iter().min().unwrap())
        .max(w.iter().max().unwrap() - w.iter().min().unwrap());
    println!("{}", ans);
}
