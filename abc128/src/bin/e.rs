use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        stxv: [(i64, i64, i64); n],
        dv: [i64; q],
    };

    let mut ev = vec![];
    for (s, t, x) in stxv {
        ev.push((s - x, 0, x));
        ev.push((t - x, 1, x));
    }
    ev.sort();

    let mut map = std::collections::BTreeMap::new();
    let mut i_e = 0;
    for d in dv {
        while i_e < n * 2 && ev[i_e].0 <= d {
            if ev[i_e].1 == 0 {
                map.entry(ev[i_e].2).and_modify(|e| *e += 1).or_insert(1);
            } else {
                map.entry(ev[i_e].2).and_modify(|e| *e -= 1);
                if map.get(&ev[i_e].2) == Some(&0) {
                    map.remove(&ev[i_e].2);
                }
            }
            i_e += 1;
        }
        let x = map.iter().nth(0).map(|(x, _)| x).unwrap_or(&-1);
        println!("{}", x);
    }
}
