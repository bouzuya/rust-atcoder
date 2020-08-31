use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut set = std::collections::BTreeSet::new();
    for &a_i in a.iter() {
        let mut m = a_i;
        while m % 2 == 0 {
            m /= 2;
        }
        set.insert(m);
    }
    let ans = set.len();
    println!("{}", ans);
}
