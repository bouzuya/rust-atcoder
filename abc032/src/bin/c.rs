use proconio::input;

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        k: i64,
        s: [i64; n],
    };
    if s.contains(&0) {
        println!("{}", n);
        return;
    }
    let mut t: Vec<(i64, i64)> = vec![];
    for &s_i in s.iter() {
        match t.last_mut() {
            Some((t_prev, c_prev)) if *t_prev == 1 && s_i == 1 => {
                *c_prev += 1;
            }
            None | Some(_) => {
                t.push((s_i, 1));
            }
        }
    }
    let m = t.len();
    let mut ans = 0;
    for l in 0..m {
        let mut c = 0;
        let mut p = 1;
        for r in l..m {
            p *= t[r].0;
            c += t[r].1;
            if p > k {
                break;
            }
            chmax!(ans, c);
        }
    }
    println!("{}", ans);
}
