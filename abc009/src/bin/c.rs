use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };
    let mut map = std::collections::BTreeMap::new();
    for &s_i in s.iter() {
        *map.entry(s_i).or_insert(0) += 1;
    }

    let mut t = vec![];
    for i in 0..n {
        let map_o = map.clone();
        for (&ch, v) in map.iter_mut() {
            if *v == 0 {
                continue;
            }
            t.push(ch);
            *v -= 1;
            let mut m = map_o.clone();
            *m.entry(ch).or_default() -= 1;
            let mut u = vec![false; n];
            for j in i + 1..n {
                let c = m.entry(s[j]).or_default();
                if *c > 0 {
                    *c -= 1;
                    u[j] = true;
                }
            }
            let count = s[0..=i]
                .iter()
                .zip(t[0..=i].iter())
                .filter(|(s_j, t_j)| s_j != t_j)
                .count()
                + u[i + 1..].iter().filter(|&&b| !b).count();
            if count <= k {
                break;
            }
            t.pop();
            *v += 1;
        }
    }

    let ans = t.iter().collect::<String>();
    println!("{}", ans);
}
