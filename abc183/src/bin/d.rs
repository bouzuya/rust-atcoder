use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i64,
        stp: [(usize, usize, i64); n],
    };
    let mut used = vec![0; 200_000 + 1];
    for (s, t, p) in stp {
        used[s] += p;
        used[t] -= p;
    }

    for i in (0..used.len()).skip(1) {
        used[i] += used[i - 1];
    }

    let ans = used.into_iter().all(|u| u <= w);
    println!("{}", if ans { "Yes" } else { "No" });
}
