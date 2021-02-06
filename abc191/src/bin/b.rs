use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n],
    };
    let mut first = true;
    for &a_i in a.iter() {
        if a_i == x {
            continue;
        }
        print!("{}{}", if first { "" } else { " " }, a_i);
        first = false;
    }
    println!();
}
