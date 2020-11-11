use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
        a: [i64; n],
    };
    let mut p = None;
    for i in 1..n {
        if a[i - 1] + a[i] >= l {
            p = Some(i);
        }
    }
    match p {
        None => {
            println!("Impossible");
        }
        Some(index) => {
            println!("Possible");
            for j in 1..index {
                println!("{}", j);
            }
            for j in (index + 1..n).rev() {
                println!("{}", j);
            }
            println!("{}", index);
        }
    }
}
