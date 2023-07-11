use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    };
    let mut d = vec![0_i64; 2 * 1_000_000 + 10];
    for (l_i, r_i) in lr {
        d[l_i * 10] += 1;
        d[r_i * 10 + 5] -= 1;
    }
    for i in 0..2 * 1_000_000 {
        d[i + 1] += d[i];
    }
    let mut ans = vec![];
    let inf = 1 << 60;
    let mut cur = None;
    for i in 0..=2 * 1_000_000 {
        if d[i] > 0 {
            cur = match cur {
                None => Some((i / 10, inf)),
                Some((s, _)) => Some((s, i / 10)),
            };
        } else {
            if let Some(w) = cur {
                ans.push(w);
            }
            cur = None;
        }
    }
    if let Some(w) = cur {
        ans.push(w);
    }

    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}
