use proconio::input;

fn main() {
    input! {
        y: usize,
        m: usize,
        d: usize,
    };
    if m == 12 && d == 25 {
        println!("{}", y - 2018);
    } else {
        println!("NOT CHRISTMAS DAY");
    }
}
