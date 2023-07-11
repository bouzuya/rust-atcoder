use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    };

    let mut ans = 0_usize;
    for r in 0..n as i64 {
        for c in 0..n as i64 {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let mut values = vec![];
                    for k in 1..(n + 1) as i64 {
                        let (i, j) = (
                            (10 * n as i64 + r + k * dr) as usize % n,
                            (10 * n as i64 + c + k * dc) as usize % n,
                        );
                        values.push(a[i][j]);
                    }
                    ans = ans.max(values.iter().collect::<String>().parse::<usize>().unwrap());
                }
            }
        }
    }

    println!("{}", ans);
}
