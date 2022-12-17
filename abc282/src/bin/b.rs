use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };
    let mut count = 0_usize;

    for i in 0..n {
        for j in i + 1..n {
            let mut ok = true;
            for k in 0..m {
                if s[i][k] != 'o' && s[j][k] != 'o' {
                    ok = false;
                    break;
                }
            }

            if ok {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
