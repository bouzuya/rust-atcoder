use proconio::{input, marker::Chars};

fn f(x: &[char], dx: i64) -> Option<u64> {
    let s = x.iter().collect::<String>();
    let k = s.len();
    for start in 1..=9 {
        let mut ok = true;
        let mut cur = start;
        let mut t = vec![cur];
        for _ in 0..k - 1 {
            cur += dx;
            if cur < 0 || cur > 9 {
                ok = false;
                break;
            }
            t.push(cur);
        }
        if ok {
            let t = t
                .iter()
                .map(|&t_i| (t_i as u8 + b'0') as char)
                .collect::<String>();
            if t >= s {
                return Some(t.parse::<u64>().unwrap());
            }
        }
    }

    None
}

fn main() {
    input! {
        x: Chars,
    };
    let s = x.iter().collect::<String>();
    let k = s.len();

    if k == 1 {
        println!("{}", s);
        return;
    }

    let d = x
        .iter()
        .copied()
        .map(|x_i| (x_i as u8 - b'0') as i64)
        .collect::<Vec<i64>>();
    let dx = d[1] - d[0];
    let mut ok = true;
    for i in 0..k - 1 {
        if d[i + 1] - d[i] != dx {
            ok = false;
            break;
        }
    }
    if ok {
        println!("{}", s);
        return;
    }
    let mut min = std::u64::MAX;
    for dx in -9..=9 {
        let a = f(&x, dx).unwrap_or(std::u64::MAX);
        min = min.min(a);
    }
    let ans = min;
    println!("{}", ans);
}
