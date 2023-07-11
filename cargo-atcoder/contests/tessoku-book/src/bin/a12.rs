use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let mut ok = 1_usize << 60;
    let mut ng = 0;
    while ok - ng > 1 {
        let x = ng + (ok - ng) / 2;
        let sum = a.iter().copied().map(|a_i| x / a_i).sum::<usize>();
        if sum >= k {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
