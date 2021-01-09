use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };
    let mut count = 1;
    let mut p = a[0];
    let mut d = None;
    for a_i in a {
        if p == a_i {
            continue;
        }
        match d {
            None => {
                if p < a_i {
                    d = Some(true);
                } else if p > a_i {
                    d = Some(false);
                }
            }
            Some(true) => {
                if p > a_i {
                    d = None;
                    count += 1;
                }
            }
            Some(false) => {
                if p < a_i {
                    d = None;
                    count += 1;
                }
            }
        }
        p = a_i;
    }
    let ans = count;
    println!("{}", ans);
}
