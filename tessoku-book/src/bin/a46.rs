use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let mut used = vec![false; n];
    used[0] = true;
    let mut ans = vec![0];
    let mut prev = 0;
    loop {
        let mut next = prev;
        let mut dist = 1_000_000_000_i64;
        let mut count = 0_usize;
        for i in 0..n {
            if used[i] {
                continue;
            }
            count += 1;

            let (px, py) = xy[prev];
            let (nx, ny) = xy[i];
            let d = (px - nx).pow(2) + (py - ny).pow(2);
            if d < dist {
                next = i;
                dist = d;
            }
        }
        if count == 0 {
            break;
        }

        used[next] = true;
        ans.push(next);
        prev = next;
    }

    ans.push(0);

    for a in ans {
        println!("{}", a + 1);
    }
}
