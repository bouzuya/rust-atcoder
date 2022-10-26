use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let d = s
        .into_iter()
        .map(|s_i| match s_i {
            'A' => 1,
            'B' => -1,
            _ => unreachable!(),
        })
        .collect::<Vec<i64>>();
    let mut min = vec![1_i64; n];
    for (i, d_i) in d.iter().copied().enumerate() {
        if d_i > 0 {
            if min[i] >= min[i + 1] {
                min[i + 1] = min[i] + 1;
            }
        }
    }
    for (i, d_i) in d.iter().copied().enumerate().rev() {
        if d_i < 0 {
            if min[i] <= min[i + 1] {
                min[i] = min[i].max(min[i + 1] + 1);
            }
        }
    }

    let ans = min.iter().sum::<i64>();
    println!("{}", ans);
}
