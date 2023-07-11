use proconio::input;

fn main() {
    input! {
        d: usize,
        g: usize,
        pc: [(usize, usize); d],
    };

    let mut min = pc.iter().map(|(p_i, _)| p_i).sum::<usize>();
    for bits in 0..1 << d {
        let b = (0..d).map(|i| (bits >> i) & 1 == 1).collect::<Vec<bool>>();

        let mut score = 0_usize;
        let mut count = 0_usize;
        for i in 0..d {
            let point = 100 * (i + 1);
            if b[i] {
                count += pc[i].0;
                score += pc[i].1 + pc[i].0 * point;
            }
        }

        for i in (0..d).rev() {
            if b[i] {
                continue;
            }

            if score >= g {
                break;
            }

            let point = 100 * (i + 1);
            if (pc[i].0 - 1) * point + score < g {
                count += pc[i].0 - 1;
                score += (pc[i].0 - 1) * point;
            } else {
                let c = (g - score + point - 1) / point;
                count += c;
                score += c * point;
            }
        }

        if score >= g {
            min = min.min(count);
        }
    }
    let ans = min;
    println!("{}", ans);
}
