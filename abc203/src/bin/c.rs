use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(usize, usize); n],
    };
    ab.sort();
    let mut c = k;
    for (a_i, b_i) in ab {
        if c >= a_i {
            c += b_i;
        } else {
            break;
        }
    }
    let ans = c;
    println!("{}", ans);
}
