use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [i64; n],
    };
    let mut map = std::collections::BTreeMap::new();
    for &t_i in t.iter() {
        *map.entry(t_i).or_insert(0) += 1_usize;
    }
    let mut c = 0;
    let mut s = 0;
    let mut p = 1;
    for (k, &v) in map.iter() {
        for _ in 0..v {
            c += k;
            s += c;
        }
        let mut p_i = 1;
        for x in 1..=v {
            p_i *= x;
            p_i %= 1_000_000_007;
        }
        p *= p_i;
        p %= 1_000_000_007;
    }
    println!("{}", s);
    println!("{}", p);
}
