use proconio::input;

fn main() {
    input! {
      a: [[usize; 3]; 3],
      n: usize,
      b: [usize; n],
    }

    let mut c = vec![vec![false; 3]; 3];
    for b_i in b {
        for j in 0..3 {
            for k in 0..3 {
                if a[j][k] == b_i {
                    c[j][k] = true;
                }
            }
        }
    }

    let ans = (0..3).any(|i| (0..3).all(|j| c[i][j]))
        || (0..3).any(|j| (0..3).all(|i| c[i][j]))
        || (0..3).all(|i| c[i][i])
        || (0..3).all(|i| c[i][3 - 1 - i]);
    println!("{}", if ans { "Yes" } else { "No" });
}
