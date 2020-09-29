use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    };

    let mut map = std::collections::BTreeMap::new();
    for r in 0..h {
        for c in 0..w {
            *map.entry(a[r][c]).or_insert(0) += 1;
        }
    }
    let mut q = std::collections::BinaryHeap::new();
    for (k, v) in map {
        q.push((v, k));
    }
    let h_e = h % 2 == 0;
    let w_e = w % 2 == 0;
    if h_e && w_e {
        // ExE
        while let Some((v, k)) = q.pop() {
            if v < 4 {
                println!("No");
                return;
            }
            if v - 4 > 0 {
                q.push((v - 4, k));
            }
        }
        println!("Yes");
    } else if (h_e && !w_e) || (!h_e && w_e) {
        // ExO or OxE
        for _ in 0..if h_e { (w - 1) * h } else { (h - 1) * w } / 4 {
            if let Some((v, k)) = q.pop() {
                if v < 4 {
                    println!("No");
                    return;
                }
                if v - 4 > 0 {
                    q.push((v - 4, k));
                }
            }
        }
        // 1xE or Ex1
        while let Some((v, k)) = q.pop() {
            if v < 2 {
                println!("No");
                return;
            }
            if v - 2 > 0 {
                q.push((v - 2, k));
            }
        }
        println!("Yes");
    } else {
        // OxO
        if h == 1 && w == 1 {
            println!("Yes");
        } else if (h == 1 && w != 1) || (h != 1 && w == 1) {
            for _ in 0..std::cmp::max(h, w) / 2 {
                if let Some((v, k)) = q.pop() {
                    if v < 2 {
                        println!("No");
                        return;
                    }
                    if v - 2 > 0 {
                        q.push((v - 2, k));
                    }
                }
            }
            println!("Yes");
        } else {
            for _ in 0..(h - 1) * (w - 1) / 4 {
                if let Some((v, k)) = q.pop() {
                    if v < 4 {
                        println!("No");
                        return;
                    }
                    if v - 4 > 0 {
                        q.push((v - 4, k));
                    }
                }
            }
            for _ in 0..h / 2 + w / 2 {
                if let Some((v, k)) = q.pop() {
                    if v < 2 {
                        println!("No");
                        return;
                    }
                    if v - 2 > 0 {
                        q.push((v - 2, k));
                    }
                }
            }
            println!("Yes");
        }
    }
}
