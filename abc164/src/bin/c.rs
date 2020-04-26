use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    };
    let mut set = std::collections::BTreeSet::new();
    for s in ss {
        set.insert(s.clone());
    }
    let ans = set.len();
    println!("{}", ans);
}
