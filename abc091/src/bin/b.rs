use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
        m: usize,
        t: [String; m],
    };

    let mut map_s = HashMap::new();
    for s_i in s {
        *map_s.entry(s_i).or_insert(0) += 1_usize;
    }

    let mut map_t = HashMap::new();
    for t_i in t {
        *map_t.entry(t_i).or_insert(0) += 1_usize;
    }

    let mut max = 0_usize;
    for (s_i, count_s) in map_s {
        let count_t = *map_t.get(&s_i).unwrap_or(&0);
        max = max.max(count_s.saturating_sub(count_t));
    }
    let ans = max;
    println!("{}", ans);
}
