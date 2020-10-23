use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    for i in 0..n {
        let mut count = 0;
        let mut x = i;
        loop {
            x = a[x];
            count += 1;
            if x == i {
                break;
            }
        }
        print!("{}{}", count, if i == n - 1 { "\n" } else { " " });
    }
}
