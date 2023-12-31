use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    for i in 0..n {
        let mut x = i;
        let mut c = 0_usize;
        loop {
            x = a[x];
            c += 1;
            if x == i {
                break;
            }
        }
        println!("{}", c);
    }
}
