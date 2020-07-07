use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    match n {
        1 => {
            println!("Hello World");
            return;
        }
        2 => {
            input! {
                a: usize,
                b: usize,
            };
            println!("{}", a + b);
        }
        _ => unreachable!(),
    }
}
