use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    if n < 357 {
        println!("0");
        return;
    }

    let mut cs: Vec<usize> = vec![];
    for len in 3..=n.to_string().len() {
        let mut curr = vec![3, 5, 7];
        for _ in 0..len - 1 {
            let mut next = vec![];
            for x in curr {
                next.push(x * 10 + 3);
                next.push(x * 10 + 5);
                next.push(x * 10 + 7);
            }
            curr = next;
        }

        cs.extend(curr.iter());
    }

    let mut count = 0;
    for c in cs {
        let s = c.to_string();
        if s.contains('3') && s.contains('5') && s.contains('7') && c <= n {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
