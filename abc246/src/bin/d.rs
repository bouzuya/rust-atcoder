use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    };
    if n == 0 {
        println!("Yes");
        return;
    }

    let mut y = 0;
    for a in 1.. {
        if a * a * a >= n {
            y = a;
            break;
        }
    }

    // WIP

    let ans = n - a.len();
    println!("{}", ans);
}
