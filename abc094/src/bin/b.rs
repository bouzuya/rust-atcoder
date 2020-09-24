use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        a: [usize; m],
    };
    let mut c = vec![0; n + 1];
    for &a_i in a.iter() {
        c[a_i] = 1;
    }
    let mut r = 0;
    for i in x..n {
        r += c[i];
    }
    let mut l = 0;
    for i in 0..x {
        l += c[i];
    }
    let ans = std::cmp::min(l, r);
    println!("{}", ans);
}
