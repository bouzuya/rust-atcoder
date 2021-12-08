use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut ans = 0_usize;
    let mut p = None;
    for a_i in a {
        match p {
            Some(c) => {
                if c != a_i {
                    p = Some(a_i);
                } else {
                    ans += 1;
                    p = None;
                }
            }
            None => {
                p = Some(a_i);
            }
        }
    }
    println!("{}", ans);
}
