use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        ty: [(usize, i64); n],
    };
    let t1s = {
        let mut t1s = vec![(0, 0, 0, vec![])];
        for (t, y) in ty {
            match t {
                1 => {
                    t1s.push((y, 0, 0, vec![]));
                }
                2 => {
                    let x = t1s.last_mut().unwrap();
                    if y >= 0 {
                        x.1 += y;
                    } else {
                        x.2 += y;
                        x.3.push(y);
                    }
                }
                _ => unreachable!(),
            }
        }
        t1s
    };

    let mut p = (0, 0, 0, vec![]);
    let mut ans = -(1_i64 << 60);
    for x in t1s.into_iter().rev() {
        let mut c3 =
            p.3.iter()
                .copied()
                .chain(x.3.into_iter())
                .collect::<Vec<i64>>();
        c3.sort();
        let a = x.0 + p.1 + p.2 + x.1 + x.2 - c3.iter().take(k).sum::<i64>();
        ans = ans.max(a);
        if k == 0 {
            break;
        }
        k -= 1;
        p = (0, p.1 + x.1, p.2 + x.2, c3);
    }
    println!("{}", ans);
}
