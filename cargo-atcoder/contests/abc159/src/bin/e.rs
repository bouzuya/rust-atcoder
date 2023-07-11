use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    };

    let mut ans = (w - 1) * (h - 1);
    'outer: for bits in 0..1 << h {
        let is = (0..h).map(|i| (bits >> i) & 1 == 1).collect::<Vec<bool>>();
        let mut lines = 0_usize;
        let mut sum = vec![0_usize; h];
        for j in 0..w {
            let mut count = vec![0_usize; h];
            let mut prev = is[0];
            let mut l = 0;
            for (i, b) in is.iter().copied().enumerate() {
                if prev != b {
                    l += 1;
                }
                count[l] += (s[i][j] as u8 - b'0') as usize;
                prev = b;
            }

            let mut ok = true;
            for i in 0..h {
                if count[i] > k {
                    continue 'outer;
                }
                if sum[i] + count[i] > k {
                    ok = false;
                }
            }
            if ok {
                for i in 0..h {
                    sum[i] += count[i];
                }
            } else {
                lines += 1;
                sum = count;
            }
        }
        let mut prev = is[0];
        for b in is.iter().copied() {
            if prev != b {
                lines += 1;
            }
            prev = b;
        }
        ans = ans.min(lines);
    }
    println!("{}", ans);
}
