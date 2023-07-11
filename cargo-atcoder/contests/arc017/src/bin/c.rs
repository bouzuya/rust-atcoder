use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        w: [i64; n]
    };
    let mut map1 = std::collections::BTreeMap::new();
    for bits in 0..1 << n / 2 {
        let mut sum = 0_i64;
        for i in 0..n {
            if (bits >> i) & 1 == 1 {
                sum += w[i];
            }
        }
        *map1.entry(sum).or_insert(0) += 1;
    }
    let mut map2 = std::collections::BTreeMap::new();
    for bits in 0..1 << (n - n / 2) {
        let mut sum = 0_i64;
        for i in 0..n {
            if (bits >> i) & 1 == 1 {
                sum += w[n / 2 + i];
            }
        }
        *map2.entry(sum).or_insert(0) += 1_i64;
    }
    let mut ans = 0_i64;
    for (k, v) in map1.iter() {
        ans += v * map2.get(&(x - k)).unwrap_or(&0);
    }
    println!("{}", ans);
}
