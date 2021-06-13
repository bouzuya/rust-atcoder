use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        k: [usize; q],
    };
    'outer: for k_i in k {
        let mut pc = 0;
        let mut p = k_i;
        loop {
            let c = a.upper_bound(&p);
            // eprintln!("{:?} {} {}", a, p, c);
            if p - c >= k_i {
                println!("{}", k_i + c);
                continue 'outer;
            } else {
                p += c - pc;
                pc = c;
            }
        }
    }
}
