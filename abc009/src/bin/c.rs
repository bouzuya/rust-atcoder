use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: i64,
        mut s: Chars,
    };

    let mut c = 0;
    let mut u = vec![false; n];
    for i in 0..n {
        let mut min_index = i;
        for j in (i + 1..n).rev() {
            if s[j] < s[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            let x = if u[min_index] && u[i] {
                0
            } else if u[min_index] || u[i] {
                1
            } else {
                2
            };
            if c + x <= k {
                c += x;
                u[min_index] = true;
                let c = s[i];
                s[i] = s[min_index];
                s[min_index] = c;
            }
        }
    }
    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
