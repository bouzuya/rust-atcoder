use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n],
    };
    a.sort();
    let mut sum = 0_usize;
    for (i, a_i) in a.into_iter().enumerate() {
        sum += a_i;
        if sum > x {
            println!("{}", i);
            return;
        }
    }
    if sum == x {
        println!("{}", n);
    } else {
        println!("{}", n - 1);
    }
}
