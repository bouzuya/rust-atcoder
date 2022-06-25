use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        a: [Usize1; k],
        l: [usize; q],
    };
    let mut b = vec![false; n];
    for a_i in a {
        b[a_i] = true;
    }
    for l_i in l {
        let mut count = 0;
        for j in 0..n {
            if b[j] {
                count += 1;
            }
            if count == l_i {
                if j + 1 < n {
                    if !b[j + 1] {
                        b[j] = false;
                        b[j + 1] = true;
                    }
                }
            }
        }
    }
    for (i, b_i) in b.iter().copied().enumerate() {
        if b_i {
            println!("{}", i + 1);
        }
    }
}
