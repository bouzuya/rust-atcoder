use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        a: [usize; n - 1],
        xy: [(Usize1, usize); m],
    };
    let mut b = vec![0; n];
    for (x, y) in xy {
        b[x] = y;
    }
    let mut cur = t;
    for (a_i, b_i) in a.iter().copied().zip(b.iter().copied().skip(1)) {
        if a_i >= cur {
            println!("No");
            return;
        }
        cur -= a_i;
        cur += b_i;
    }
    println!("Yes");
}
