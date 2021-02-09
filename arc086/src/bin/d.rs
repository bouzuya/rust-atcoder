use proconio::input;

fn print_ans(ans: &Vec<(usize, usize)>) {
    println!("{}", ans.len());
    for &(x, y) in ans.iter() {
        println!("{} {}", x + 1, y + 1);
    }
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let min: i64 = *a.iter().min().unwrap();
    let max: i64 = *a.iter().max().unwrap();
    if min >= 0 {
        assert!(min >= 0 && max >= 0);
        let mut ans = vec![];
        for i in 0..n - 1 {
            ans.push((i, i + 1));
        }
        print_ans(&ans);
        return;
    } else {
        assert!(min < 0);
        if max >= 0 {
            assert!(min < 0 && max >= 0);
            if min.abs() <= max.abs() {
                let i_max = a.iter().position(|&x| x == max).unwrap();
                let mut ans = vec![];
                for i in 0..n {
                    if i == i_max {
                        continue;
                    }
                    ans.push((i_max, i));
                }
                for i in 0..n - 1 {
                    ans.push((i, i + 1));
                }
                print_ans(&ans);
                return;
            } else {
                let i_min = a.iter().position(|&x| x == min).unwrap();
                let mut ans = vec![];
                for i in 0..n {
                    if i == i_min {
                        continue;
                    }
                    ans.push((i_min, i));
                }
                for i in (1..n).rev() {
                    ans.push((i, i - 1));
                }
                print_ans(&ans);
                return;
            }
        } else {
            assert!(min < 0 && max < 0);
            let mut ans = vec![];
            for i in (1..n).rev() {
                ans.push((i, i - 1));
            }
            print_ans(&ans);
            return;
        }
    }
}
