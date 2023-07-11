use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    };

    if a >= b {
        println!("0");
        return;
    }

    let mut c = a;
    let mut count = 0;
    while c < b {
        count += 1;
        match c.checked_mul(k) {
            None => {
                break;
            }
            Some(d) => c = d,
        }
    }
    let ans = count;
    println!("{}", ans);
}
