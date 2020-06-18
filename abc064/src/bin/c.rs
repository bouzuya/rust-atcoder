use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let colors = vec![
        (1..=399),
        (400..=799),
        (800..=1199),
        (1200..=1599),
        (1600..=1999),
        (2000..=2399),
        (2400..=2799),
        (2800..=3199),
    ];
    let mut set = std::collections::BTreeSet::new();
    let mut w = 0;
    for &a_i in a.iter() {
        match colors.iter().position(|r| r.contains(&a_i)) {
            Some(p) => {
                set.insert(p);
            }
            None => w += 1,
        }
    }
    println!("{} {}", std::cmp::max(1, set.len()), set.len() + w);
}
