use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    };
    let mut ok = 0;
    let mut ng = n;
    while ng - ok > 1 {
        let mid = ok + (ng - ok) / 2;
        if a[mid] <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = ok + 1;
    println!("{}", ans);
}
