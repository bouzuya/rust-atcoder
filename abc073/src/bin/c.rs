use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut set = std::collections::BTreeSet::new();
    for &a_i in a.iter() {
        if set.contains(&a_i) {
            set.remove(&a_i);
        } else {
            set.insert(a_i);
        }
    }
    let ans = set.len();
    println!("{}", ans);
}
