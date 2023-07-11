use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut count = 0;
    for x in 1..=555555 {
        let digits = x.to_string().chars().collect::<Vec<char>>();
        if digits.iter().all(|&c| c == digits[0]) {
            count += 1;
        }
        if count == n {
            println!("{}", x);
            return;
        }
    }
}
