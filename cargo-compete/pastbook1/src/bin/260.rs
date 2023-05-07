use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_by_key(|&(_, b)| b);

    let mut count = 0_usize;
    let mut cur = 0_usize;
    for (a, b) in ab {
        if cur < a {
            cur = b;
            count += 1;
        }
    }

    let ans = count;
    println!("{}", ans);
}
