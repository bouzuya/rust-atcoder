use num::Rational;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    };

    let mut a = vec![];
    for (x, y) in xy {
        let a1 = if x - 1 == 0 {
            Rational::new(std::isize::MAX, 1)
        } else {
            Rational::new(y, x - 1)
        };
        let a2 = Rational::new(y - 1, x);
        a.push((a1, a2));
    }
    a.sort();

    let mut count = 0_usize;
    let mut p = Rational::new(-1, 1);
    for (a1, a2) in a {
        if a2 < p {
            continue;
        }
        count += 1;
        p = a1;
    }

    let ans = count;
    println!("{}", ans);
}
