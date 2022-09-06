use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n]
    };

    for _ in 0..4 {
        let mut t = vec![vec![' '; n]; n];
        for i in 0..n {
            for j in 0..n {
                t[i][j] = s[j][n - 1 - i];
            }
        }
        s = t;

        for i in 0..n {
            for j in 0..n {
                let mut ok = true;
                let mut count = 0;
                for k in 0..6 {
                    if j + k >= n {
                        ok = false;
                        break;
                    }
                    if s[i][j + k] == '.' {
                        count += 1;
                    }
                }
                if ok && count <= 2 {
                    println!("Yes");
                    return;
                }

                let mut ok = true;
                let mut count = 0;
                for k in 0..6 {
                    if i + k >= n {
                        ok = false;
                        break;
                    }
                    if s[i + k][j] == '.' {
                        count += 1;
                    }
                }
                if ok && count <= 2 {
                    println!("Yes");
                    return;
                }

                let mut ok = true;
                let mut count = 0;
                for k in 0..6 {
                    if i + k >= n || j + k >= n {
                        ok = false;
                        break;
                    }
                    if s[i + k][j + k] == '.' {
                        count += 1;
                    }
                }
                if ok && count <= 2 {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
