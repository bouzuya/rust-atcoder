use proconio::input;

macro_rules! chmin {
    ($e: expr, $v: expr) => {
        if $v < $e {
            $e = $v;
        }
    };
}

fn main() {
    input! {
        d: usize,
        g: i64,
        pc: [(i64, i64); d],
    };
    let mut ans = pc.iter().map(|(p_i, _)| p_i).sum::<i64>();
    for bits in 0..1 << d {
        let mut use_c = vec![false; d];
        for i in 0..d {
            use_c[i] = (bits >> i) & 1 == 1;
        }
        let mut score = 0;
        let mut count = 0;
        for (i, &(p_i, c_i)) in pc.iter().enumerate() {
            if !use_c[i] {
                continue;
            }
            let s_i = 100 * (i + 1) as i64;
            score += c_i + p_i * s_i;
            count += p_i;
        }
        if score >= g {
            chmin!(ans, count);
        } else {
            for (i, &(p_i, c_i)) in pc.iter().enumerate().rev() {
                if use_c[i] {
                    continue;
                }
                let s_i = 100 * (i + 1) as i64;
                let p = std::cmp::min(((g - score) + (s_i - 1)) / s_i, p_i);
                score += p * s_i + if p == p_i { c_i } else { 0 };
                count += p;
                if score >= g {
                    break;
                }
            }
            chmin!(ans, count);
        }
    }
    println!("{}", ans);
}
