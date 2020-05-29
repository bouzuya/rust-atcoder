use proconio::input;

fn main() {
    input! {
        h: i64,
        w: i64,
        n: usize,
        ab: [(i64, i64); n]
    };
    let mut map = std::collections::BTreeMap::new();
    for &(y, x) in ab.iter() {
        for ny in std::cmp::max(2, y - 1)..=std::cmp::min(y + 1, h - 1) {
            for nx in std::cmp::max(2, x - 1)..=std::cmp::min(x + 1, w - 1) {
                *map.entry((ny, nx)).or_insert(0) += 1;
            }
        }
    }
    let mut c = vec![0; 9 + 1];
    for &v in map.values() {
        c[v] += 1;
    }
    c[0] = (h - 2) * (w - 2) - c.iter().sum::<i64>();
    for &c_i in c.iter() {
        println!("{}", c_i);
    }
}
