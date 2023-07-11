use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut chars = "123456".chars().collect::<Vec<char>>();
    let r1 = (n / 5) % 6;
    let r2 = n % 5;
    chars[0..].rotate_left(r1);
    for i in 0..r2 {
        let c = chars[i];
        chars[i] = chars[i + 1];
        chars[i + 1] = c;
    }
    for c in chars.iter() {
        print!("{}", c);
    }
    println!();
}
