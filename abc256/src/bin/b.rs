use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut c = vec![0; 4];
    let mut p = 0_usize;
    c[0] = 1;
    for a_i in a {
        c[0] = 1;
        for j in (0..4).rev() {
            if j + a_i < 4 {
                c[j + a_i] = c[j];
            } else {
                p += c[j];
            }
        }
        for j in 0..a_i {
            c[j] = 0;
        }
        // c[a_i - 1] = 1;
    }
    let ans = p;
    println!("{}", ans);
}
