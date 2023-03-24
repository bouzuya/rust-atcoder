use proconio::input;

fn main() {
    input! {
        mut h: usize,
        mut m: usize,
    };
    loop {
        let hhmm = format!("{:02}{:02}", h, m).chars().collect::<Vec<char>>();
        let a = hhmm[0];
        let b = hhmm[1];
        let c = hhmm[2];
        let d = hhmm[3];
        let h2 = format!("{}{}", a, c).parse::<usize>().unwrap();
        let m2 = format!("{}{}", b, d).parse::<usize>().unwrap();
        if (0..=23).contains(&h2) && (0..=59).contains(&m2) {
            println!("{} {}", h, m);
            break;
        }

        m += 1;
        if m == 60 {
            m = 0;
            h += 1;
        }
        if h == 24 {
            h = 0;
        }
    }
}
