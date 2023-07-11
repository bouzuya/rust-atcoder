use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut map = std::collections::BTreeMap::new();
    for &a_i in a.iter() {
        map.entry(a_i).or_insert(0);
    }
    for (i, (_, v)) in map.iter_mut().enumerate() {
        *v = i;
    }
    for &a_i in a.iter() {
        println!("{}", map.get(&a_i).unwrap());
    }
}
