fn main() {
    let d1 = captcha_sum("1122");
    let d2 = captcha_halfway_sum("1212");
    println!("{}", d1);
    println!("{}", d2);
}

fn captcha_sum(s: &str) -> u32 {
    let split: Vec<u32> = s.split("")
        .map(|s| s.parse())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect();

    let len = split.len();
    let mut sum = 0;
    for i in 0..len {
        let j = (i + 1) % len;
        if split[i] == split[j] {
            sum += split[i];
        }
    }

    return sum;
}

fn captcha_halfway_sum(s: &str) -> u32 {
    let split: Vec<u32> = s.split("")
        .map(|s| s.parse())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap())
        .collect();

    let len = split.len();
    let offset = len / 2;
    let mut sum = 0;
    for i in 0..len {
        let j = (i + offset) % len;
        if split[i] == split[j] {
            sum += split[i];
        }
    }

    return sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn captach_sum_test() {
        assert!(captcha_sum("1122") == 3);
    }

    #[test]
    fn captach_hlafway_sum_test() {
        assert!(captcha_halfway_sum("1212") == 6);
    }
}