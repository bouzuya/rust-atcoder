use proconio::input;

fn choose(n: usize, r: usize) -> usize {
    if n < r {
        return 0;
    }
    if r == 0 {
        return 1;
    }
    let mut m = 1;
    for i in 0..r {
        m *= n - i;
        m /= i + 1;
    }
    m
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        mut v: [usize; n],
    };

    v.sort();
    v.reverse();

    let avg = v.iter().take(a).sum::<usize>() as f64 / a as f64;

    let mut count_eqmin = 0_usize;
    let mut count_fixed = 0_usize;
    for (i, v_i) in v.iter().copied().enumerate() {
        if v_i == v[a - 1] {
            count_eqmin += 1;
            if i < a {
                count_fixed += 1;
            }
        }
    }

    let count = if v[0] == v[a - 1] {
        (a..=b).map(|x| choose(count_eqmin, x)).sum::<usize>()
    } else {
        choose(count_eqmin, count_fixed)
    };

    println!("{}", avg);
    println!("{}", count);
}
