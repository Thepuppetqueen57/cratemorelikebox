use std::time::{self, Duration, Instant, SystemTime, UNIX_EPOCH};

fn get_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

struct BasicRng {
    seed: u128,
}

impl BasicRng {
    pub fn new() -> Self {
        BasicRng {
            seed: get_timestamp(),
        }
    }

    pub fn next(&mut self) -> u128 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 17;
        self.seed ^= self.seed << 5;
        self.seed
    }
}

pub fn shuffle(vec: &mut Vec<char>, rounds: u32) {
    let mut rng = BasicRng::new();

    for _ in 0..rounds {
        let ri1 = rng.next() as usize % vec.len();
        let ri2 = rng.next() as usize % vec.len();

        vec.swap(ri1, ri2);
    }
}

pub fn craddler() -> String {
    "This project is using the best lib ever fr fr".to_string()
}

pub fn craddlerbutcooler(cooler: i32) -> i32 {
    cooler + 3
}

pub fn craddle() -> String {
    "What if you put craddler but without the second R 😃".to_string()
}

pub fn thegruddle(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    shuffle(&mut chars, 20);
    chars.iter().collect()
}

pub fn cruddler(num: i32) -> i32 {
    let mut numstr = num.to_string();
    numstr.insert(0, '-');
    let numnum: i32 = numstr.parse().unwrap();

    numnum
}

pub fn tobble(string1: &str, string2: &str) -> String {
    string1.to_string() + string2
}

pub fn crubble() -> String {
    "linus torvalds".to_string()
}

pub fn brabble(tf: bool) -> bool {
    !tf
}

pub fn warddle(name: &str) -> String {
    "you have angered the gods ".to_string() + name
}

pub fn dabble(numberone: i32, numbertwo: i32) -> i32 {
    numberone / numbertwo
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
        let awesomeawesomer = craddle();
        assert_eq!(
            awesomeawesomer,
            "What if you put craddler but without the second R 😃"
        );
    }
    #[test]
    fn boxtest4() {
        let amazingamazer = thegruddle("hello");
        assert_ne!(amazingamazer, "hello");
    }

    #[test]
    fn boxtest5() {
        let LudicrousCrimesCapturedOnBankSecurityCamerasTheCulpritStillHasNotBeenCaught =
            cruddler(3);
        assert_eq!(
            LudicrousCrimesCapturedOnBankSecurityCamerasTheCulpritStillHasNotBeenCaught,
            -3
        );
    }

    #[test]
    fn boxtest6() {
        let delightfuldelighter = tobble("hi ", "boys");
        assert_eq!(delightfuldelighter, "hi boys");
    }

    #[test]
    fn boxtest7() {
        let delectabledelecter = crubble();
        assert_eq!(delectabledelecter, "linus torvalds");
    }

    #[test]
    fn boxtest8() {
        let tastytaster = brabble(true);
        assert_eq!(tastytaster, false);
    }

    #[test]
    fn boxtest9() {
        let crappycrapper = warddle("barth");
        assert_eq!(crappycrapper, "you have angered the gods barth");
    }

    #[test]
    fn boxtest10() {
        let deliciousdelighter = dabble(6, 2);
        assert_eq!(deliciousdelighter, 3);
    }
}
