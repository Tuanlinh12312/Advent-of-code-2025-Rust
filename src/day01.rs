type Input = Vec<(char, i32)>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| (line.chars().nth(0).unwrap(), line[1..].parse().unwrap()))
        .collect()
}

pub fn p1(input: &Input) -> i32 {
    let mut ans = 0;
    let mut orientation = 50;
    for &(direction, rotation) in input.iter() {
        match direction {
            'L' => orientation -= rotation,
            'R' => orientation += rotation,
            _ => panic!("Unknown direction"),
        }

        orientation %= 100;
        if orientation < 0 {
            orientation += 100;
        }

        if orientation == 0 {
            ans += 1;
        }
    }
    ans
}

pub fn p2(input: &Input) -> i32 {
    let mut ans = 0;
    let mut orientation = 50;
    for &(direction, rotation) in input.iter() {
        match direction {
            'L' => {
                ans += (rotation + (100 - orientation)%100).div_euclid(100);
                orientation -= rotation;
            },
            'R' => {
                ans += (rotation + orientation).div_euclid(100);
                orientation += rotation;
            },
            _ => panic!("Unknown direction"),
        }

        orientation = orientation.rem_euclid(100);
    }
    ans
}