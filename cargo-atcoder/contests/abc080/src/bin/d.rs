use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        capital_c: usize,
        stc: [(usize, usize, Usize1); n],
    };
    let mut count = vec![0_i64; 200_000 + 2 + 1];
    for c in 0..capital_c {
        let mut count_c = vec![0_i64; 200_000 + 2 + 1];
        for (s, t, c_i) in stc.iter().copied() {
            if c_i != c {
                continue;
            }
            count_c[2 * s - 1] += 1;
            count_c[2 * t] -= 1;
        }
        for i in 0..count_c.len() - 1 {
            count_c[i + 1] += count_c[i];
        }
        for i in 0..count_c.len() - 1 {
            if count_c[i] > 0 {
                count[i] += 1;
            }
        }
    }

    let ans = count.iter().max().unwrap();
    println!("{}", ans);
}
