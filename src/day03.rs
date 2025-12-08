type Input = Vec<Vec<u32>>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn p1(input: &Input) -> u32 {
    let mut ans = 0;
    for battery in input.iter() {
        let n = battery.len();
        let mut mval = 0;
        let mut mpos = n;

        for i in (0..=n - 2).rev() {
            if battery[i] >= mval {
                mval = battery[i];
                mpos = i;
            }
        }

        let mval2 = *battery[mpos + 1..].iter().max().unwrap();
        ans += mval * 10 + mval2;
    }
    ans
}

pub fn p2(input: &Input) -> u64 {
    let mut ans = 0u64;
    for battery in input.iter() {
        let n = battery.len();
        let mut num = 0u64;
        let mut left = 0;

        for digitsLeft in (0..12).rev() {
            let mut mval = 0;
            let mut mpos = 0;

            for i in (left..n-digitsLeft).rev() {
                if battery[i] >= mval {
                    mval = battery[i];
                    mpos = i;
                }
            }

            num = num*10 + u64::from(mval);
            left = mpos+1;
        }
        ans += num;
    }
    ans
}
