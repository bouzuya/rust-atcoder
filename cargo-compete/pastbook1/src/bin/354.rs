use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut ans = vec![0_i64; m];
    let mut kids = vec![0_usize; n];
    for (i, a_i) in a.iter().copied().enumerate() {
        let index = kids.upper_bound(&(a_i - 1));
        if index > 0 {
            kids[index - 1] = a_i;
            ans[i] = (n - (index - 1)) as i64;
        } else {
            ans[i] = -1;
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
