use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    // n の約数を列挙する
    let mut d = vec![];
    for i in 1.. {
        if i * i > m {
            break;
        }
        if m % i == 0 {
            d.push(i);
            if i != m / i {
                d.push(m / i);
            }
        }
    }

    let mut ans = 1;
    for &d_i in d.iter() {
        if n * d_i <= m {
            ans = std::cmp::max(ans, d_i);
        }
    }
    println!("{}", ans);
}
