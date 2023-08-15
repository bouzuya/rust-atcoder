use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    };
    if l % 2019 == 0 || r % 2019 == 0 {
        println!("0");
        return;
    }

    if r / 2019 - l / 2019 > 1 {
        println!("0");
        return;
    }

    let mut min = (l * r) % 2019;
    for i in l..=r {
        for j in i + 1..=r {
            min = min.min((i * j) % 2019);
        }
    }

    let ans = min;
    println!("{}", ans);
}
