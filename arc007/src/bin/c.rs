use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut c: Chars,
    };
    let n = c.len();
    while c[0] != 'o' {
        c.rotate_left(1);
    }
    let mut ans = 1;
    let mut x = c.clone();
    for i in 0..n {
        if x[i] == 'o' {
            continue;
        }
        ans += 1;
        let mut min_count = 10;
        let mut min_j = 0;
        for j in 0..n {
            let mut d = c.clone();
            d.rotate_left(j);
            if d[0] != 'o' {
                continue;
            }
            let mut count = 0;
            for k in 0..n - i {
                if x[i + k] == 'x' && d[k] == 'x' {
                    count += 1;
                }
            }
            if count < min_count {
                min_count = count;
                min_j = j;
            }
        }
        let mut d = c.clone();
        d.rotate_left(min_j);
        for k in 0..n - i {
            if d[k] == 'o' {
                x[i + k] = 'o';
            }
        }
    }
    println!("{}", ans);
}
