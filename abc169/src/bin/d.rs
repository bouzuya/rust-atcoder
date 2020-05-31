use proconio::input;

fn main() {
    input! {
        n: u64,
    };

    let mut p = vec![];
    let mut m = n;
    for i in 2.. {
        if i * i > m {
            break;
        }
        let mut count = 0;
        while m % i == 0 {
            count += 1;
            m /= i;
        }
        p.push((i, count));
    }
    if m != 1 {
        p.push((n, 1));
    }
    let mut count = 0;
    for i in 1.. {
        let mut used = false;
        for p_i in p.iter_mut() {
            if p_i.1 >= i {
                p_i.1 -= i;
                used = true;
                count += 1;
            }
        }
        if !used {
            break;
        }
    }

    let ans = count;
    println!("{}", ans);
}
