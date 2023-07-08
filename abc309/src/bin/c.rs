use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(usize, usize); n],
    };
    let mut sum = ab.iter().copied().map(|(_, b)| b).sum::<usize>();
    ab.sort_by_key(|&(a, _)| a);
    if sum <= k {
        println!("1");
        return;
    }

    let mut ans = 0;
    for (a, b) in ab {
        ans = a + 1;
        sum -= b;
        if sum <= k {
            break;
        }
    }
    println!("{}", ans);
}
