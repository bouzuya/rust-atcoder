use num_rational::Ratio;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let dx = xy[j].0 - xy[i].0;
                let dy = xy[j].1 - xy[i].1;
                if dx == 0 {
                    if xy[i].0 == xy[k].0 {
                        println!("Yes");
                        return;
                    } else {
                        continue;
                    }
                }
                let a = Ratio::new(dy, dx);
                let b = Ratio::new(xy[i].1, 1) - a * Ratio::new(xy[i].0, 1);
                if (a * Ratio::new(xy[i].0, 1) + b != Ratio::new(xy[i].1, 1))
                    || (a * Ratio::new(xy[j].0, 1) + b != Ratio::new(xy[j].1, 1))
                {
                    panic!();
                }
                if a * Ratio::new(xy[k].0, 1) + b == Ratio::new(xy[k].1, 1) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
