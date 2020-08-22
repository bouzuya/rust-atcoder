use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        k: usize,
        mut t: [usize; n],
    };
    t.sort();

    let mut ans = 0;
    let mut i_l = 0;
    let mut i_r = 0;
    while i_l < n {
        while i_r + 1 < n && t[i_r + 1] <= t[i_l] + k {
            i_r += 1;
        }
        ans += 1;
        i_l += c;
        if i_l > i_r {
            i_l = i_r + 1;
        }
    }
    println!("{}", ans);
}
