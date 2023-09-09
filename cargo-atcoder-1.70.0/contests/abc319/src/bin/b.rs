use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for i in 0..=n {
        let mut found = false;
        for j in 1..=9 {
            if n % j != 0 {
                continue;
            }
            if i % (n / j) == 0 {
                print!("{}", j);
                found = true;
                break;
            }
        }
        if !found {
            print!("-");
        }
    }
    println!();
}
