use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        n: i64,
    };
    for i in n.. {
        if i % a == 0 && i % b == 0 {
            println!("{}", i);
            return;
        }
    }
}
