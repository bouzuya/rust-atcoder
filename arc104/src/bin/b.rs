use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    // ACGT
    let mut count = vec![vec![0; 4]; n + 1];
    for (i, &c_i) in s.iter().enumerate() {
        for j in 0..4 {
            count[i + 1][j] = count[i][j];
        }
        let j = ['A', 'C', 'G', 'T'].iter().position(|&c| c == c_i).unwrap();
        count[i + 1][j] += 1;
    }

    let mut ans = 0;
    for l in 0..n {
        for r in l + 1..=n {
            let c_a = count[r][0] - count[l][0];
            let c_c = count[r][1] - count[l][1];
            let c_g = count[r][2] - count[l][2];
            let c_t = count[r][3] - count[l][3];
            if c_a == c_t && c_c == c_g {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
