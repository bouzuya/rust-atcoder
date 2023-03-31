use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    };

    let mut levels = vec![(0_usize, 0_usize); n + 1];
    levels[0] = (1, 1);
    for i in 1..=n {
        let (a, p) = levels[i - 1];
        levels[i] = (3 + a * 2, 1 + p * 2);
    }

    let mut level = n;
    let mut offset = 0_usize;
    let mut sum = 0_usize;
    loop {
        let (a, p) = levels[level];
        if x == offset + a {
            sum += p;
            break;
        } else if x == offset + 1 {
            assert!(level != 0);
            break;
        } else {
            let (pa, pp) = levels[level - 1];
            let mid = 1 + pa + 1;
            match x.cmp(&(offset + mid)) {
                std::cmp::Ordering::Less => {
                    offset += 1;
                    level -= 1;
                }
                std::cmp::Ordering::Equal => {
                    sum += pp + 1;
                    break;
                }
                std::cmp::Ordering::Greater => {
                    sum += pp + 1;
                    offset += mid;
                    level -= 1;
                }
            }
        }
    }

    let ans = sum;
    println!("{}", ans);
}
