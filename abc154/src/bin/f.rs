use proconio::input;

pub struct ModCombin {
    factv: Vec<usize>,
    finvv: Vec<usize>,
    modp: usize,
}

impl ModCombin {
    pub fn new(max: usize, modp: usize) -> Self {
        let mut factv = vec![0usize; max + 1];
        let mut finvv = vec![0usize; max + 1];
        factv[0] = 1;
        for i in 1..=max {
            factv[i] = (factv[i - 1] * i) % modp;
        }
        finvv[max] = Self::inv(factv[max], modp);
        for i in (1..=max).rev() {
            finvv[i - 1] = (finvv[i] * i) % modp;
        }
        Self { factv, finvv, modp }
    }

    pub fn combination(&self, n: usize, r: usize) -> usize {
        if n < r {
            return 0;
        }
        ((((self.factv[n] * self.finvv[r]) % self.modp) * self.finvv[n - r]) % self.modp)
    }

    // private
    fn inv(n: usize, modp: usize) -> usize {
        Self::pow(n, (modp - 2) as u32, modp)
    }

    // private
    fn pow(n: usize, mut x: u32, modp: usize) -> usize {
        let mut b = n;
        let mut a = 1usize;
        while x > 1 {
            if x & 1 == 1 {
                a = (a * b) % modp;
            }
            x = x / 2;
            b = (b * b) % modp;
        }
        if x == 1 {
            a = (a * b) % modp;
        }
        a
    }
}

fn g(mc: &ModCombin, r: usize, c: usize) -> usize {
    return (mc.modp + mc.combination((r + 1) + (c + 1), r + 1) - 1) % mc.modp;
}

fn main() {
    input! {
        r1: usize,
        c1: usize,
        r2: usize,
        c2: usize
    };
    let modp = 1_000_000_007;
    let mc = ModCombin::new(2_000_005, modp);
    let ans = (modp + modp + g(&mc, r2, c2) - g(&mc, r1 - 1, c2) - g(&mc, r2, c1 - 1)
        + g(&mc, r1 - 1, c1 - 1))
        % modp;
    println!("{}", ans);
}
