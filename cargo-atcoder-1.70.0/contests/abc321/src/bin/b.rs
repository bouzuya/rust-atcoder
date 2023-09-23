use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: [usize; n - 1],
    };
    a.sort();
    let sum = a.iter().copied().sum::<usize>();
    let min = a.iter().copied().min().unwrap();
    let max = a.iter().copied().max().unwrap();
    let cur = sum - min - max;
    if cur + max < x {
        println!("-1");
        return;
    }

    for y in 0..=100 {
        let new = if y > max {
            cur + max
        } else if min <= y && y <= max {
            cur + y
        } else if y < min {
            cur + min
        } else {
            unreachable!()
        };
        if new >= x {
            println!("{}", y);
            return;
        }
    }
}
