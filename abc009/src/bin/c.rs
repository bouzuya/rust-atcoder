use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut k: i64,
        mut s: Chars,
    };

    let mut u = vec![false; n];
    for i in 0..n {
        if k > 0 {
            let mut min_index = i;
            for j in i + 1..n {
                if s[j] < s[min_index] {
                    min_index = j;
                }
            }
            if min_index != i {
                k -= if u[min_index] && u[i] {
                    0
                } else if u[min_index] || u[i] {
                    1
                } else {
                    2
                };
                u[min_index] = true;
                if k >= 0 {
                    let c = s[i];
                    s[i] = s[min_index];
                    s[min_index] = c;
                }
            }
        }
    }
    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
