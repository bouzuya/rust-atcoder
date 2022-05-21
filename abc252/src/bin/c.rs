use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let mut ans = 1_usize << 60;
    for i in 0_u8..=9 {
        let c = (b'0' + i) as char;

        let mut counts = vec![0; 10];
        for s_j in s.iter() {
            counts[s_j.iter().position(|s_jk| s_jk == &c).unwrap()] += 1;
        }

        let mut time = 0_usize;
        let mut j = 0;
        while !counts.iter().all(|&count| count == 0) {
            if counts[j] > 0 {
                counts[j] -= 1;
            }
            time += 1;
            j += 1;
            j %= 10;
        }

        ans = ans.min(time.saturating_sub(1));
    }
    println!("{}", ans);
}
