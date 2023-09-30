use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    };

    let mut ans = vec![n + 1; n];
    for a_i in a {
        ans[a_i] = 0;
    }
    let mut count = 0_usize;
    for i in (0..n).rev() {
        if ans[i] == 0 {
            count = 0;
        } else {
            count += 1;
            ans[i] = count;
        }
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
