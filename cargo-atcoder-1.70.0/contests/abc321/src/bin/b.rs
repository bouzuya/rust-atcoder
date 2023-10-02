use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n - 1],
    };
    for a_n in 0..=100 {
        let mut a = a.clone();
        a.push(a_n);
        a.sort();
        if a[1..n - 1].iter().copied().sum::<usize>() >= x {
            println!("{}", a_n);
            return;
        }
    }
    println!("-1");
}
