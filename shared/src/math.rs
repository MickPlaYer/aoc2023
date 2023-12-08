pub fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

pub fn lcm_n(numbers: Vec<usize>) -> usize {
    if numbers.is_empty() {
        return 1;
    }
    let mut result = numbers[0];
    for &num in &numbers[1..] {
        result = lcm(result, num);
    }
    result
}

pub fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}
