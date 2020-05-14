use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut map = std::collections::BTreeMap::new();
    for &a_i in a.iter() {
        *map.entry(a_i).or_insert(0) += 1;
    }
    let entries = map
        .iter()
        .filter(|(_, &v)| v >= 2)
        .map(|(&k, &v)| (k, v))
        .collect::<Vec<(i64, i64)>>();
    let mut k1 = None;
    for &(k, v) in entries.iter().rev() {
        if v >= 4 {
            let ans = match k1 {
                Some(x) => x * k,
                None => k * k,
            };
            println!("{}", ans);
            return;
        } else if v >= 2 {
            match k1 {
                Some(x) => {
                    println!("{}", x * k);
                    return;
                }
                None => {
                    k1 = Some(k);
                }
            }
        }
    }
    println!("{}", 0);
}
