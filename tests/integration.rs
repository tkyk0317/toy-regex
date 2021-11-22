#[cfg(test)]
mod test {
    use toy_regex::regex::Regex;

    #[test]
    fn test_integration1() {
        let re = Regex::new("a?bc");

        assert_eq!(true, re.exec("bc", true, true));
        assert_eq!(true, re.exec("bc", true, false));
        assert_eq!(true, re.exec("bc", false, true));
        assert_eq!(true, re.exec("bc", false, false));

        assert_eq!(true, re.exec("abc", true, true));
        assert_eq!(true, re.exec("abc", true, false));
        assert_eq!(true, re.exec("abc", false, true));
        assert_eq!(true, re.exec("abc", false, false));

        assert_eq!(true, re.exec("aabc", true, true));
        assert_eq!(false, re.exec("aabc", true, false));
        assert_eq!(true, re.exec("aabc", false, true));
        assert_eq!(false, re.exec("aabc", false, false));

        assert_eq!(false, re.exec("", true, true));
        assert_eq!(false, re.exec("", true, false));
        assert_eq!(false, re.exec("", false, true));
        assert_eq!(false, re.exec("", false, false));

        assert_eq!(false, re.exec("ab", true, true));
        assert_eq!(false, re.exec("ab", true, false));
        assert_eq!(false, re.exec("ab", false, true));
        assert_eq!(false, re.exec("ab", false, false));

        assert_eq!(false, re.exec("aab", true, true));
        assert_eq!(false, re.exec("aab", true, false));
        assert_eq!(false, re.exec("aab", false, true));
        assert_eq!(false, re.exec("aab", false, false));
    }

    #[test]
    fn test_integration2() {
        let re = Regex::new("abc");

        assert_eq!(true, re.exec("abc", true, true));
        assert_eq!(true, re.exec("abc", true, false));
        assert_eq!(true, re.exec("abc", false, true));
        assert_eq!(true, re.exec("abc", false, false));

        assert_eq!(true, re.exec("aabc", true, true));
        assert_eq!(false, re.exec("aabc", true, false));
        assert_eq!(true, re.exec("aabc", false, true));
        assert_eq!(false, re.exec("aabc", false, false));

        assert_eq!(false, re.exec("", true, true));
        assert_eq!(false, re.exec("", true, false));
        assert_eq!(false, re.exec("", false, true));
        assert_eq!(false, re.exec("", false, false));

        assert_eq!(false, re.exec("ab", true, true));
        assert_eq!(false, re.exec("ab", true, false));
        assert_eq!(false, re.exec("ab", false, true));
        assert_eq!(false, re.exec("ab", false, false));

        assert_eq!(false, re.exec("aab", true, true));
        assert_eq!(false, re.exec("aab", true, false));
        assert_eq!(false, re.exec("aab", false, true));
        assert_eq!(false, re.exec("aab", false, false));
    }

    #[test]
    fn test_integration3() {
        let re = Regex::new("a+b+");

        assert_eq!(true, re.exec("abc", true, true));
        assert_eq!(true, re.exec("abc", true, false));
        assert_eq!(true, re.exec("abc", false, true));
        assert_eq!(false, re.exec("abc", false, false));

        assert_eq!(true, re.exec("zabc", true, true));
        assert_eq!(false, re.exec("zabc", true, false));
        assert_eq!(true, re.exec("zabc", false, true));
        assert_eq!(false, re.exec("zabc", false, false));

        assert_eq!(true, re.exec("aaaaaaabbbbbbbc", true, true));
        assert_eq!(true, re.exec("aaaaaaabbbbbbbc", true, false));
        assert_eq!(true, re.exec("aaaaaaabbbbbbbc", false, true));
        assert_eq!(false, re.exec("aaaaaaabbbbbbbc", false, false));
    }
}
