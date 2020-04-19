use proconio::input;

fn main() {
    input! {
        n: usize,
        spv: [(String, i64); n],
    };
    let mut v: Vec<(&String, i64, usize)> = spv
        .iter()
        .enumerate()
        .map(|(i, (s, p))| (s, -p, i + 1))
        .collect();
    v.sort();
    for (_, _, ans) in v {
        println!("{}", ans);
    }
}
