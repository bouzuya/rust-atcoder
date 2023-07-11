use proconio::input;

fn main() {
    input! {
        x: i128,
        a: i128,
        d: i128,
        n: i128,
    };

    if d == 0 {
        let ans = (x - a).abs();
        println!("{}", ans);
        return;
    }

    if d > 0 {
        let min = a;
        let max = a + d * (n - 1);

        if x > max {
            println!("{}", (max - x).abs());
            return;
        } else if x < min {
            println!("{}", (min - x).abs());
            return;
        } else {
            let y = (x - min) % d;
            println!("{}", y.min(d - y));
            return;
        }
    }

    if d < 0 {
        let min = a + d * (n - 1);
        let max = a;
        if x > max {
            println!("{}", (max - x).abs());
            return;
        } else if x < min {
            println!("{}", (min - x).abs());
            return;
        } else {
            let y = (x - min) % d.abs();
            println!("{}", y.min(d.abs() - y));
            return;
        }
    }
}
