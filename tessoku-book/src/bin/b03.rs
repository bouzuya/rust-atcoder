use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut ok = false;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if a[i] + a[j] + a[k] == 1_000 {
                    ok = true;
                }
            }
        }
    }
    let ans = ok;
    println!("{}", if ans { "Yes" } else { "No" });
}
