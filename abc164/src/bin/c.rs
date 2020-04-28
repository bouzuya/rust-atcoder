use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    };
    let ans = ss
        .iter()
        .map(|s| s)
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    println!("{}", ans);
}
