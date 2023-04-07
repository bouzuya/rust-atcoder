use proconio::input;

fn main() {
    input! {
      n: usize,
    }

    let mut count = 0_usize;
    for length in 1.. {
        for d in 1..=9 {
            count += 1;
            if count == n {
                println!("{}", d.to_string().repeat(length));
                return;
            }
        }
    }
}
