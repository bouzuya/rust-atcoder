use std::collections::{BTreeMap, BTreeSet};

use proconio::{input, marker::Chars};

fn dfs(map: &mut BTreeMap<char, u8>, s: &[Vec<char>], i: usize, j: usize, c: u8) -> bool {
    if i == s[2].len() {
        return c == 0
            && s[0]
                .iter()
                .map(|c| (map[c] + b'0') as char)
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
                != 0
            && s[1]
                .iter()
                .map(|c| (map[c] + b'0') as char)
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
                != 0
            && map[&s[0][0]] != 0
            && map[&s[1][0]] != 0
            && map[&s[2][0]] != 0;
    }

    match j {
        0 | 1 => {
            let n = s[j].len();
            if i < n {
                for d1 in 0..=9 {
                    let p3 = s[j][n - 1 - i];
                    if let Some(&d) = map.get(&p3) {
                        if d1 != d {
                            continue;
                        }
                        if dfs(map, s, i, (j + 1) % 3, c) {
                            return true;
                        }
                    } else {
                        if map.values().any(|v| v == &d1) {
                            continue;
                        }
                        map.insert(p3, d1);
                        if dfs(map, s, i, (j + 1) % 3, c) {
                            return true;
                        }
                        map.remove(&p3);
                    }
                }
                false
            } else {
                dfs(map, s, i, (j + 1) % 3, c)
            }
        }
        2 => {
            let n1 = s[0].len();
            let n2 = s[1].len();
            let n3 = s[2].len();
            let d1 = if i < n1 { map[&s[0][n1 - 1 - i]] } else { 0 };
            let d2 = if i < n2 { map[&s[1][n2 - 1 - i]] } else { 0 };
            let ds = d1 + d2 + c;
            let p3 = s[2][n3 - 1 - i];
            if let Some(&d3) = map.get(&p3) {
                if ds % 10 != d3 {
                    return false;
                }
                if dfs(map, s, i + 1, 0, ds / 10) {
                    return true;
                }
                false
            } else {
                if map.values().any(|v| v == &(ds % 10)) {
                    return false;
                }
                map.insert(p3, ds % 10);
                if dfs(map, s, i + 1, 0, ds / 10) {
                    return true;
                }
                map.remove(&p3);
                false
            }
        }
        _ => unreachable!(),
    }
}

fn main() {
    input! {
        s: [Chars; 3],
    };

    if s[0]
        .iter()
        .copied()
        .chain(s[1].iter().copied())
        .chain(s[2].iter().copied())
        .collect::<BTreeSet<_>>()
        .len()
        > 10
    {
        println!("UNSOLVABLE");
        return;
    }

    let mut map = BTreeMap::new();
    if dfs(&mut map, &s, 0, 0, 0) {
        for s_i in s {
            for s_ij in s_i {
                print!("{}", map[&s_ij]);
            }
            println!();
        }
    } else {
        println!("UNSOLVABLE");
    }
}
