// サンプルが通らず
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        ty: [(usize, i64); n],
    };
    let t1s = {
        let mut t1s = vec![(0, 0, vec![])];
        for (t, y) in ty {
            match t {
                1 => {
                    t1s.push((y, 0, vec![]));
                }
                2 => {
                    let (mut x0, mut x1, mut x2) = t1s.pop().unwrap();
                    if y >= 0 {
                        x0 += y;
                    } else {
                        x1 += y;
                        x2.push(y);
                    }
                    t1s.push((x0, x1, x2));
                }
                _ => unreachable!(),
            }
        }
        t1s
    };

    let mut p1 = 0;
    let mut p2 = vec![];
    let mut ans = -(1_i64 << 60);
    for (x0, x1, x2) in t1s.into_iter().rev() {
        let c1 = p1 + x1;
        let mut c2 = p2
            .iter()
            .copied()
            .chain(x2.into_iter())
            .collect::<Vec<i64>>();
        c2.sort();
        ans = ans.max(x0 + c1 - c2.iter().take(k).sum::<i64>());
        if k == 0 {
            break;
        }
        k -= 1;
        p1 = c1;
        p2 = c2;
    }
    println!("{}", ans);
}
