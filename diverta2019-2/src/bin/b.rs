use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let mut map = std::collections::BTreeMap::new();
    for (i, &(x_i, y_i)) in xy.iter().enumerate() {
        for (j, &(x_j, y_j)) in xy.iter().enumerate() {
            if i == j {
                continue;
            }
            *map.entry((x_j - x_i, y_j - y_i)).or_insert(0) += 1;
        }
    }
    let mut c = vec![];
    for (&k, &v) in map.iter() {
        c.push((v, k.0, k.1));
    }
    c.sort_by_key(|(c, _, _)| -c);
    let ans = n as i64 - c.first().unwrap_or(&(0, 0, 0)).0;
    println!("{:?}", ans);
}
