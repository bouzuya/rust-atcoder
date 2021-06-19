use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut ok = n;
    let mut ng = 0;
    while ok - ng > 1 {
        let i = (ok + ng) / 2;
        let x = (1 + i).checked_mul(i).map(|x| x / 2);
        if let Some(x) = x {
            if x >= n {
                ok = i;
            } else {
                ng = i;
            }
        } else {
            ok = i;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
