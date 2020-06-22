use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        bc: [(i64, i64); q]
    };

    let mut sum = 0_i64;
    let mut map = std::collections::BTreeMap::new();
    for &a_i in a.iter() {
        *map.entry(a_i).or_insert(0) += 1_i64;
        sum += a_i;
    }

    for &(b_i, c_i) in bc.iter() {
        if map.contains_key(&b_i) {
            let count = map[&b_i];
            sum += count * c_i;
            sum -= count * b_i;
            map.remove(&b_i);
            *map.entry(c_i).or_insert(0) += count;
        }
        println!("{}", sum);
    }
}
