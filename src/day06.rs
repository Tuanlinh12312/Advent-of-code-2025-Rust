type Input = String;

pub fn parse(input: &str) -> Input {
    input.to_string()
}

pub fn p1(input: &Input) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let sz = lines.len();
    let nums: Vec<_> = lines[0..sz - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let ops = lines
        .iter()
        .nth_back(0)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<_>>();

    let mut ans = 0;
    for i in 0..nums[0].len() {
        let mut cnt = nums[0][i];
        for j in 1..nums.len() {
            if ops[i] == '*' {
                cnt *= nums[j][i];
            } else {
                cnt += nums[j][i];
            }
        }
        ans += cnt;
    }
    ans
}

pub fn p2(input: &Input) -> usize {
    let chars: Vec<Vec<char>> = input
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    let sz = chars.len();

    let mut res = 0usize;
    for i in 0..chars[sz - 1].len() {
        let op = chars[sz - 1][i];
        if op == ' ' {
            continue;
        }

        let mut ans = if op == '*' { 1usize } else { 0usize };
        for j in i..chars[sz - 1].len() {
            let mut num = 0usize;
            for k in 0..(sz - 1) {
                let digit = chars[k][j].to_digit(10);
                if let Some(dig) = digit {
                    num = num * 10 + (dig as usize);
                }
            }

            if num == 0 {
                break;
            }
            ans = if op == '*' { ans * num } else { ans + num };
        }

        res += ans;
    }
    res
}
