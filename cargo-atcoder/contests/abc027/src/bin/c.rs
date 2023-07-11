use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let depth = {
        let mut d = 0;
        let mut m = n;
        while m > 0 {
            m /= 2;
            d += 1;
        }
        d
    };
    let mut t = true;
    let mut l = depth % 2 == 0;
    let mut x = 1_usize;
    while x <= n {
        if l {
            x = x * 2;
        } else {
            x = x * 2 + 1;
        }
        t = !t;
        l = !l;
    }
    println!("{}", if t { "Takahashi" } else { "Aoki" });
}
