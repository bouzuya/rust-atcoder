use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [[usize; 3]; n],
    };
    let mut s = vec![0; n];
    for i in 0..n {
        for j in 0..3 {
            s[i] += p[i][j];
        }
    }

    let mut count = vec![0; 1200 + 1 + 1];
    for i in 0..n {
        count[s[i]] += 1;
    }

    for i in (1..=900).rev() {
        count[i - 1] += count[i];
    }

    for i in 0..n {
        let ans = count[(s[i] + 300) + 1] < k;
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
