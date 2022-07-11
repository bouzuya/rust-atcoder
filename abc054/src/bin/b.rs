use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; n],
        b: [Chars; m],
    };
    for oy in 0..n - m + 1 {
        for ox in 0..n - m + 1 {
            let mut ok = true;
            for i in 0..m {
                for j in 0..m {
                    if a[oy + i][ox + j] != b[i][j] {
                        ok = false;
                    }
                }
            }
            if ok {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
