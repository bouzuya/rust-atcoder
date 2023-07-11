use proconio::input;

fn main() {
    input! {
        q: usize,
        l: usize,
    };
    let mut size = 0_usize;
    let mut stack = vec![];
    for _ in 0..q {
        input! { o: String };
        match o.as_str() {
            "Push" => {
                input! { n: usize, m: i64 };
                stack.push((n, m));
                size += n;
                if size > l {
                    println!("FULL");
                    return;
                }
            }
            "Pop" => {
                input! { n: usize };
                if size < n {
                    println!("EMPTY");
                    return;
                }
                let mut x = n;
                while let Some((n, m)) = stack.pop() {
                    size -= n;
                    if n > x {
                        stack.push((n - x, m));
                        size += n - x;
                        break;
                    }
                    if n == x {
                        break;
                    }
                    assert!(n < x);
                    x -= n;
                }
            }
            "Top" => {
                if size == 0 {
                    println!("EMPTY");
                    return;
                }
                let (_, m) = stack.last().unwrap();
                println!("{}", m);
            }
            "Size" => println!("{}", size),
            _ => unreachable!(),
        }
    }
    println!("SAFE");
}
