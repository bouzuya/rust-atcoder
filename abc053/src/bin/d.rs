use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let set = a.into_iter().collect::<std::collections::BTreeSet<i64>>();
    let ans = set.len() - if set.len() % 2 == 0 { 1 } else { 0 };
    println!("{}", ans);
}
