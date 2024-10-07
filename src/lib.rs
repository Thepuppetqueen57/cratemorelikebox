use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn craddler() -> String {
    return "This project is using the best lib ever fr fr".to_string();
}

pub fn craddlerbutcooler(cooler: i32) -> i32 {
    return cooler + 3;
}

pub fn craddle() -> String {
    return "What if you put craddler but without the R 😃".to_string();
}

pub fn thegruddle(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    let mut rng = thread_rng();
    chars.shuffle(&mut rng);
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boxtest() {
        let epicepik = craddler();
        assert_eq!(epicepik, "This project is using the best lib ever fr fr");
    }

    #[test]
    fn boxtest2() {
        let coolcooler = craddlerbutcooler(3);
        assert_eq!(coolcooler, 6);
    }

    #[test]
    fn boxtest3() {
        let awesomeawesoming = craddle();
        assert_eq!(awesomeawesoming, "What if you put craddler but without the R 😃");
    }

    #[test]
    fn boxtest4() {
        let amazingamazer = thegruddle("hello");
        assert_ne!(amazingamazer, "hello");
    }
}