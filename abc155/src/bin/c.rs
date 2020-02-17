use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        sv: [String; n]
    };

    let mut om: BTreeMap<String, usize> = BTreeMap::new();
    for s in sv {
        *om.entry(s).or_insert(0) += 1;
    }
    let max_v = *om.values().max().unwrap();
    for s in om.into_iter().filter(|&(_, v)| v == max_v).map(|(k, _)| k) {
        println!("{}", s);
    }
}
