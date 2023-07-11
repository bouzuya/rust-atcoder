use std::collections::BTreeMap;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut map_s = BTreeMap::new();
    for s_i in s.iter() {
        *map_s.entry(s_i).or_insert(0) += 1;
    }
    let mut map_t = BTreeMap::new();
    for t_i in t.iter() {
        *map_t.entry(t_i).or_insert(0) += 1;
    }

    if map_s != map_t {
        println!("NO");
        return;
    }

    let p = s
        .iter()
        .zip(t.iter())
        .filter(|(s_i, t_i)| s_i != t_i)
        .map(|(&s_i, &t_i)| (s_i, t_i))
        .collect::<Vec<(char, char)>>();

    if p.len() > 6 {
        println!("NO");
        return;
    }

    match p.len() {
        0 | 3 | 4 | 5 => println!(
            "{}",
            if map_s.iter().filter(|(_, &v)| v >= 2).count() > 0 {
                "YES"
            } else {
                "NO"
            }
        ),
        2 | 6 => println!("YES"),
        1 => unreachable!(),
        _ => unreachable!(),
    }
}
