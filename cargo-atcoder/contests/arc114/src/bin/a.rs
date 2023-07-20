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
        x: [usize; n],
    };

    let ps = sieve_of_eratosthenes(50);
    assert_eq!(ps.len(), 15);

    let mut ans = ps.iter().copied().product::<usize>();
    for bits in 0..1 << ps.len() {
        let qs = (0..ps.len())
            .filter(|i| ((bits >> i) & 1) == 1)
            .map(|i| ps[i])
            .collect::<Vec<usize>>();
        if x.iter()
            .copied()
            .all(|x_i| qs.iter().copied().any(|q_i| x_i % q_i == 0))
        {
            ans = ans.min(qs.iter().copied().product::<usize>());
        }
    }

    println!("{}", ans);
}
