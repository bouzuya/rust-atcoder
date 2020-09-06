use proconio::input;

fn main() {
    input! {
        n: usize,
        f: [[usize; 10]; n],
        p: [[i64; 11]; n],
    };
    let mut max_score = -1_000_000_000_000_000_i64;
    for bits in 0..1 << 10 {
        let mut ls = vec![];
        for l in 0..10 {
            if (bits >> l) & 1 == 1 {
                ls.push(l);
            }
        }
        if ls.is_empty() {
            continue;
        }

        let mut score = 0_i64;
        for i in 0..n {
            let f_i = &f[i];
            let p_i = &p[i];
            let mut count = 0;
            for &l in ls.iter() {
                if f_i[l] == 1 {
                    count += 1;
                }
            }
            score += p_i[count];
        }
        max_score = std::cmp::max(max_score, score);
    }
    let ans = max_score;
    println!("{}", ans);
}
