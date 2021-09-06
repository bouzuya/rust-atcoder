use proconio::input;

macro_rules! chmin {
    ($min_v: expr, $v: expr) => {
        if $v < $min_v {
            $min_v = $v;
            true
        } else {
            false
        }
    };
}

fn f(l: usize, r: usize) -> f64 {
    ((1 + (if l > r { l - r } else { r - l }).pow(2)) as f64).sqrt()
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let s = a.iter().sum::<usize>();
    let inf = 1_000_000_000_000_f64;
    let mut curr = vec![vec![inf; s + 1]; s + 1];
    curr[0][0] = 0_f64;
    for _ in 1..n {
        let mut next = vec![vec![inf; s + 1]; s + 1];
        for sum in 0..=s {
            for j_next in 0..=sum {
                for j_prev in 0..=sum {
                    chmin!(
                        next[sum][j_next],
                        curr[sum - j_next][j_prev] + f(j_prev, j_next)
                    );
                }
            }
        }
        curr = next;
    }

    let ans = curr[s][0];
    println!("{}", ans);
}
