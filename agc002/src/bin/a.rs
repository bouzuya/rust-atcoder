use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };

    if (a..=b).contains(&0) {
        println!("Zero");
        return;
    }

    if (a > 0) || (((a - b).abs() + 1) % 2 == 0) {
        println!("Positive");
    } else {
        println!("Negative");
    }
}
