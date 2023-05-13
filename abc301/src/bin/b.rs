use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    for i in 0..n - 1 {
        if (a[i] - a[i + 1]).abs() <= 1 {
            println!("{}", a[i]);
            continue;
        }

        if a[i] < a[i + 1] {
            for a_j in a[i]..a[i + 1] {
                println!("{}", a_j);
            }
        } else {
            for a_j in (a[i + 1] + 1..=a[i]).rev() {
                println!("{}", a_j);
            }
        }
    }
    println!("{}", a[n - 1]);
}
