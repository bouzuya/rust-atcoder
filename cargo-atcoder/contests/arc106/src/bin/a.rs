use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for a in 1.. {
        match 3_usize.checked_pow(a) {
            None => break,
            Some(p3a) => {
                if p3a >= n {
                    break;
                }
                for b in 1.. {
                    match 5_usize.checked_pow(b) {
                        None => break,
                        Some(p5b) => {
                            if p5b >= n {
                                break;
                            }
                            match p3a.checked_add(p5b) {
                                None => break,
                                Some(x) => {
                                    if x == n {
                                        println!("{} {}", a, b);
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("-1");
}
