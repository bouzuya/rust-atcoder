use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        ab: [(usize, char); n],
    };
    let ans = ab
        .into_iter()
        .map(|(a, b)| match b {
            'E' => l - a,
            'W' => a,
            _ => unreachable!(),
        })
        .max()
        .unwrap();
    println!("{}", ans);
}
