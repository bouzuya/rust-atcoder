use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [Usize1; k],
        l: [Usize1; q],
    };
    for l_i in l {
        if a[l_i] == n - 1 {
            // do nothing
        } else if l_i + 1 < k {
            if a[l_i] + 1 == a[l_i + 1] {
                // do nothing
            } else {
                a[l_i] += 1;
            }
        } else {
            a[l_i] += 1;
        }
    }
    for a_i in a {
        println!("{}", a_i + 1);
    }
}
