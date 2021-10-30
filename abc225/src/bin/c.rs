use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[usize; m]; n],
    };
    if !(0..(7 - m + 1)).contains(&((b[0][0] - 1) % 7)) {
        println!("No");
        return;
    }

    let mut c = vec![vec![0; m]; n];
    c[0][0] = b[0][0];
    for i in 0..n {
        if i + 1 < n {
            c[i + 1][0] = c[i][0] + 7;
        }
        for j in 0..m - 1 {
            c[i][j + 1] = c[i][j] + 1;
        }
    }
    let ans = b == c;
    println!("{}", if ans { "Yes" } else { "No" });
}
