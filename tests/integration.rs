#[cfg(test)]
mod test {
    use toy_regex::regex::Regex;

    #[test]
    fn test_integration1() {
        let re = Regex::new("a?bc");

        assert!(re.exec("bc", true, true));
        assert!(re.exec("bc", true, false));
        assert!(re.exec("bc", false, true));
        assert!(re.exec("bc", false, false));

        assert!(re.exec("abc", true, true));
        assert!(re.exec("abc", true, false));
        assert!(re.exec("abc", false, true));
        assert!(re.exec("abc", false, false));

        assert!(re.exec("aabc", true, true));
        assert!(!re.exec("aabc", true, false));
        assert!(re.exec("aabc", false, true));
        assert!(!re.exec("aabc", false, false));

        assert!(!re.exec("", true, true));
        assert!(!re.exec("", true, false));
        assert!(!re.exec("", false, true));
        assert!(!re.exec("", false, false));

        assert!(!re.exec("ab", true, true));
        assert!(!re.exec("ab", true, false));
        assert!(!re.exec("ab", false, true));
        assert!(!re.exec("ab", false, false));

        assert!(!re.exec("aab", true, true));
        assert!(!re.exec("aab", true, false));
        assert!(!re.exec("aab", false, true));
        assert!(!re.exec("aab", false, false));
    }

    #[test]
    fn test_integration2() {
        let re = Regex::new("abc");

        assert!(re.exec("abc", true, true));
        assert!(re.exec("abc", true, false));
        assert!(re.exec("abc", false, true));
        assert!(re.exec("abc", false, false));

        assert!(re.exec("aabc", true, true));
        assert!(!re.exec("aabc", true, false));
        assert!(re.exec("aabc", false, true));
        assert!(!re.exec("aabc", false, false));

        assert!(!re.exec("", true, true));
        assert!(!re.exec("", true, false));
        assert!(!re.exec("", false, true));
        assert!(!re.exec("", false, false));

        assert!(!re.exec("ab", true, true));
        assert!(!re.exec("ab", true, false));
        assert!(!re.exec("ab", false, true));
        assert!(!re.exec("ab", false, false));

        assert!(!re.exec("aab", true, true));
        assert!(!re.exec("aab", true, false));
        assert!(!re.exec("aab", false, true));
        assert!(!re.exec("aab", false, false));
    }

    #[test]
    fn test_integration3() {
        let re = Regex::new("a+b+");

        assert!(re.exec("abc", true, true));
        assert!(re.exec("abc", true, false));
        assert!(re.exec("abc", false, true));
        assert!(!re.exec("abc", false, false));

        assert!(re.exec("zabc", true, true));
        assert!(!re.exec("zabc", true, false));
        assert!(re.exec("zabc", false, true));
        assert!(!re.exec("zabc", false, false));

        assert!(re.exec("aaaaaaabbbbbbbc", true, true));
        assert!(re.exec("aaaaaaabbbbbbbc", true, false));
        assert!(re.exec("aaaaaaabbbbbbbc", false, true));
        assert!(!re.exec("aaaaaaabbbbbbbc", false, false));
    }

    #[test]
    fn test_integration4() {
        {
            let re = Regex::new("(ab)c");

            assert!(re.exec("zabcz", true, true));
            assert!(!re.exec("zabcz", true, false));
            assert!(re.exec("zabcz", false, true));
            assert!(!re.exec("zabcz", false, false));

            assert!(!re.exec("ac", true, true));
            assert!(!re.exec("ac", true, false));
            assert!(!re.exec("ac", false, true));
            assert!(!re.exec("ac", false, false));
        }
        {
            let re = Regex::new("a|c");

            assert!(re.exec("a", true, true));
            assert!(re.exec("a", true, false));
            assert!(re.exec("a", false, true));
            assert!(re.exec("a", false, false));

            assert!(re.exec("c", true, true));
            assert!(re.exec("c", true, false));
            assert!(re.exec("c", false, true));
            assert!(re.exec("c", false, false));

            assert!(!re.exec("b", true, true));
            assert!(!re.exec("b", true, false));
            assert!(!re.exec("b", false, true));
            assert!(!re.exec("b", false, false));
        }
        {
            let re = Regex::new("(ab)*");

            assert!(re.exec("ab", true, true));
            assert!(re.exec("ab", true, false));
            assert!(re.exec("ab", false, true));
            assert!(re.exec("ab", false, false));

            assert!(re.exec("ababab", true, true));
            assert!(re.exec("ababab", true, false));
            assert!(re.exec("ababab", false, true));
            assert!(re.exec("ababab", false, false));

            assert!(re.exec("aa", true, true));
            assert!(re.exec("aa", true, false));
            assert!(re.exec("aa", false, true));
            assert!(!re.exec("aa", false, false));
        }
        {
            let re = Regex::new("(ac)|(bd)");

            assert!(re.exec("ac", true, true));
            assert!(re.exec("ac", true, false));
            assert!(re.exec("ac", false, true));
            assert!(re.exec("ac", false, false));

            assert!(re.exec("bd", true, true));
            assert!(re.exec("bd", true, false));
            assert!(re.exec("bd", false, true));
            assert!(re.exec("bd", false, false));

            assert!(!re.exec("bc", true, true));
            assert!(!re.exec("bc", true, false));
            assert!(!re.exec("bc", false, true));
            assert!(!re.exec("bc", false, false));
        }
        {
            let re = Regex::new("(ab)+");

            assert!(re.exec("ab", true, true));
            assert!(re.exec("ab", true, false));
            assert!(re.exec("ab", false, true));
            assert!(re.exec("ab", false, false));

            assert!(re.exec("abababab", true, true));
            assert!(re.exec("abababab", true, false));
            assert!(re.exec("abababab", false, true));
            assert!(re.exec("abababab", false, false));

            assert!(!re.exec("a", true, true));
            assert!(!re.exec("a", true, false));
            assert!(!re.exec("a", false, true));
            assert!(!re.exec("a", false, false));

            assert!(!re.exec("", true, true));
            assert!(!re.exec("", true, false));
            assert!(!re.exec("", false, true));
            assert!(!re.exec("", false, false));
        }
        {
            let re = Regex::new("(ab)?");

            assert!(re.exec("ab", true, true));
            assert!(re.exec("ab", true, false));
            assert!(re.exec("ab", false, true));
            assert!(re.exec("ab", false, false));

            assert!(re.exec("abababab", true, true));
            assert!(re.exec("abababab", true, false));
            assert!(re.exec("abababab", false, true));
            assert!(!re.exec("abababab", false, false));

            assert!(re.exec("", true, true));
            assert!(re.exec("", true, false));
            assert!(re.exec("", false, true));
            assert!(re.exec("", false, false));
        }
    }
}
