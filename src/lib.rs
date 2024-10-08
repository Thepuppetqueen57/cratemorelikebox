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

pub fn cruddler(num: i32) -> i32 {
    let mut numstr = num.to_string();
    numstr.insert(0, '-');
    let numnum: i32 = numstr.parse().unwrap();

    return numnum;
}

pub fn tobble(string1: &str, string2: &str) -> String {
    let stringer = string1.to_string() + string2;
    return stringer;
}

pub fn crubble() -> String {
    return "linus torvalds".to_string();
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
        assert_eq!(awesomeawesomer, "What if you put craddler but without the R 😃");
    }

    #[test]
    fn boxtest4() {
        let amazingamazer = thegruddle("hello");
        assert_ne!(amazingamazer, "hello");
    }

    #[test]
    fn boxtest5() {
        let LudicrousCrimesCapturedOnBankSecurityCamerasTheCulpritStillHasNotBeenCaught = cruddler(3);
        assert_eq!(LudicrousCrimesCapturedOnBankSecurityCamerasTheCulpritStillHasNotBeenCaught, -3);
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
}