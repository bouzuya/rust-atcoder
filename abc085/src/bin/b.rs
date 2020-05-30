use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [u64; n],
    };
    let set = d.iter().collect::<std::collections::BTreeSet<_>>();
    let ans = set.len();
    println!("{}", ans);
}
