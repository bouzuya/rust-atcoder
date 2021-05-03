use proconio::input;

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut p = vec![];
    let mut b = vec![true; n + 1];
    for i in 2.. {
        if i * i > n {
            for j in i..=n {
                if b[j] {
                    p.push(j);
                }
            }
            break;
        }
        if b[i] {
            p.push(i);
            for j in (i + i..=n).step_by(i) {
                b[j] = false;
            }
        }
    }
    p
}

fn main() {
    input! {
        n: usize,
    };
    assert!(5 <= n && n <= 55);
    // a_i <= 55_555
    // a_{i + 0} + a_{i + 1} + a_{i + 2} + a_{i + 3} + a_{i + 4} = S_i
    // a_i % 5 == 1 => S_i % 5 == 0
    let ps = sieve_of_eratosthenes(55_555);
    let mut a = vec![];
    for p_i in ps {
        if p_i >= 5 && p_i % 5 == 1 {
            a.push(p_i);
            if a.len() == n {
                break;
            }
        }
    }

    let mut ans = String::new();
    for (i, &a_i) in a.iter().enumerate() {
        ans += &format!("{}{}", a_i, if i == n - 1 { "\n" } else { " " });
    }
    print!("{}", ans);
}
