use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };
    let mut index = (0..n).collect::<Vec<usize>>();
    loop {
        let mut ok = true;

        for i in 1..n {
            let a = &s[index[i - 1]];
            let b = &s[index[i]];
            let mut count = 0_usize;
            for j in 0..m {
                if a[j] != b[j] {
                    count += 1;
                }
            }
            if count != 1 {
                ok = false;
            }
        }

        if ok {
            println!("Yes");
            return;
        }

        if !index.next_permutation() {
            break;
        }
    }
    println!("No");
}
