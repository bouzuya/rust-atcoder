use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        n: usize,
    };
    for x in n.. {
        if x % a == 0 && x % b == 0 {
            println!("{}", x);
            break;
        }
    }
}
