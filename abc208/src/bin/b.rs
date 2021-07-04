use proconio::input;

fn main() {
    input! {
        p: usize,
    };
    let mut coins = vec![];
    for i in 1..=10 {
        let mut x = 1;
        for j in 1..=i {
            x *= j;
        }
        for _ in 0..100 {
            coins.push(x);
        }
    }

    coins.sort();
    coins.reverse();

    let mut count = 0;
    let mut x = p;
    for c in coins {
        if c <= x {
            x -= c;
            count += 1;
        }
        if x == 0 {
            break;
        }
    }

    let ans = count;
    println!("{}", ans);
}
