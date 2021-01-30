use proconio::input;

fn main() {
    input! {
        mut ab: [i64; 2],
        c: usize,
    };
    let mut i = c;
    loop {
        if ab[i] == 0 {
            println!("{}", if i == 0 { "Aoki" } else { "Takahashi" });
            return;
        }
        ab[i] -= 1;
        i = if i == 0 { 1 } else { 0 };
    }
}
