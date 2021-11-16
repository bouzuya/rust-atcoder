use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            ab: [(Usize1, Usize1); m],
        };
        let mut is = vec![0; n];
        let mut os = vec![0; n];
        for (a, b) in ab {
            os[a] += 1;
            is[b] += 1;
        }
        let mut sum = 0_usize;
        for (i, o) in is.into_iter().zip(os.into_iter()) {
            sum += i.min(o);
        }
        let ans = m - sum;
        println!("{}", ans);
    }
}
