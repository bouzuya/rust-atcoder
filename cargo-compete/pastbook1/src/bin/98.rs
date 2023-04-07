use proconio::input;

fn main() {
    input! {
      k: usize,
      a: usize,
      b: usize
    }

    for x in a..=b {
        if x % k == 0 {
            println!("OK");
            return;
        }
    }
    println!("NG");
}
