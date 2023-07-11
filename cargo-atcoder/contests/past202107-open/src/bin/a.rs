use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    };
    let a = s
        .iter()
        .copied()
        .map(|s_i| (s_i - b'0') as usize)
        .collect::<Vec<usize>>();
    let ans = ((a.iter().copied().take(14).step_by(2).sum::<usize>() * 3
        + a.iter().copied().take(14).skip(1).step_by(2).sum::<usize>())
        % 10)
        == a[14];
    println!("{}", if ans { "Yes" } else { "No" });
}
