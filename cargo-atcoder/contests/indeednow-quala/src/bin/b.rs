use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let mut t = std::collections::BTreeMap::new();
    for c in "indeednow".chars() {
        *t.entry(c).or_insert(0) += 1;
    }

    for s_i in s.iter() {
        let mut m = std::collections::BTreeMap::new();
        for &s_ij in s_i.iter() {
            *m.entry(s_ij).or_insert(0) += 1;
        }
        let ans = t == m;
        println!("{}", if ans { "YES" } else { "NO" });
    }
}
