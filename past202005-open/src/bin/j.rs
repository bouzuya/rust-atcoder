use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; m],
    };
    let inf = 1_000_000_000_000;
    let mut c = vec![inf; n];
    let mut count = 0;

    for &a_i in a.iter() {
        let i = c[n - count..].upper_bound(&(a_i - 1));
        if i == 0 {
            if count < n {
                count += 1;
                c[n - count] = a_i;
                println!("{}", count);
            } else {
                println!("-1");
            }
        } else {
            c[n - count + i - 1] = a_i;
            println!("{}", count - i + 1);
        }
    }
}
