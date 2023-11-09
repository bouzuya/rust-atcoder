use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
        m: usize,
    };

    let x = x
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let d = x.iter().max().unwrap();

    fn f(x: &[usize], n: usize) -> Option<usize> {
        let mut v = 0_usize;
        for c in x.iter().copied() {
            v = v.checked_mul(n)?;
            v = v.checked_add(c)?;
        }
        Some(v)
    }

    if x.len() == 1 {
        if let Some(v) = f(&x, d + 1) {
            if v <= m {
                println!("1");
                return;
            }
        }
        println!("0");
        return;
    }

    if let Some(v) = f(&x, d + 1) {
        if v > m {
            println!("0");
            return;
        }
    } else {
        println!("0");
        return;
    }

    let mut ok = d + 1;
    let mut ng = 1 << 60;
    while ng - ok > 1 {
        let mid = ok + (ng - ok) / 2;
        if let Some(v) = f(&x, mid) {
            if v <= m {
                ok = mid;
            } else {
                ng = mid;
            }
        } else {
            ng = mid;
        }
    }
    let ans = ok - d;
    println!("{}", ans);
}
