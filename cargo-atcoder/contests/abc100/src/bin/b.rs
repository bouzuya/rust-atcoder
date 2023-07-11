use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
    };
    let mut count = 0_usize;
    for x in 1.. {
        let mut c = 0_usize;
        let mut y = x;
        while y % 100 == 0 {
            y /= 100;
            c += 1;
        }
        if c == d {
            count += 1;
        }

        if count == n {
            println!("{}", x);
            break;
        }
    }
}
