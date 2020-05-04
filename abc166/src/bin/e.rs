use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    // i + a[i] = j - a[j] (i < j) を満たす (i, j) の個数
    let mut ans = 0_usize;
    let mut m = std::collections::BTreeMap::new();
    for (i, &a_i) in a.iter().enumerate() {
        ans += m.get(&(i as i64 - a_i)).unwrap_or(&0);
        *m.entry(i as i64 + a_i).or_insert(0) += 1;
    }
    println!("{}", ans);
}
