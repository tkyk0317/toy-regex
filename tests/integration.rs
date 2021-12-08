#[cfg(test)]
mod test {
    use toy_regex::regex::Regex;

    #[test]
    fn test_integration1() {
        let re = Regex::new("a?bc");

        assert!(re.exec(Some("bc".to_string()), true, true, None));
        assert!(re.exec(Some("bc".to_string()), true, false, None));
        assert!(re.exec(Some("bc".to_string()), false, true, None));
        assert!(re.exec(Some("bc".to_string()), false, false, None));

        assert!(re.exec(Some("abc".to_string()), true, true, None));
        assert!(re.exec(Some("abc".to_string()), true, false, None));
        assert!(re.exec(Some("abc".to_string()), false, true, None));
        assert!(re.exec(Some("abc".to_string()), false, false, None));

        assert!(re.exec(Some("aabc".to_string()), true, true, None));
        assert!(!re.exec(Some("aabc".to_string()), true, false, None));
        assert!(re.exec(Some("aabc".to_string()), false, true, None));
        assert!(!re.exec(Some("aabc".to_string()), false, false, None));

        assert!(!re.exec(Some("".to_string()), true, true, None));
        assert!(!re.exec(Some("".to_string()), true, false, None));
        assert!(!re.exec(Some("".to_string()), false, true, None));
        assert!(!re.exec(Some("".to_string()), false, false, None));

        assert!(!re.exec(Some("ab".to_string()), true, true, None));
        assert!(!re.exec(Some("ab".to_string()), true, false, None));
        assert!(!re.exec(Some("ab".to_string()), false, true, None));
        assert!(!re.exec(Some("ab".to_string()), false, false, None));

        assert!(!re.exec(Some("aab".to_string()), true, true, None));
        assert!(!re.exec(Some("aab".to_string()), true, false, None));
        assert!(!re.exec(Some("aab".to_string()), false, true, None));
        assert!(!re.exec(Some("aab".to_string()), false, false, None));
    }

    #[test]
    fn test_integration2() {
        let re = Regex::new("abc");

        assert!(re.exec(Some("abc".to_string()), true, true, None));
        assert!(re.exec(Some("abc".to_string()), true, false, None));
        assert!(re.exec(Some("abc".to_string()), false, true, None));
        assert!(re.exec(Some("abc".to_string()), false, false, None));

        assert!(re.exec(Some("aabc".to_string()), true, true, None));
        assert!(!re.exec(Some("aabc".to_string()), true, false, None));
        assert!(re.exec(Some("aabc".to_string()), false, true, None));
        assert!(!re.exec(Some("aabc".to_string()), false, false, None));

        assert!(!re.exec(Some("".to_string()), true, true, None));
        assert!(!re.exec(Some("".to_string()), true, false, None));
        assert!(!re.exec(Some("".to_string()), false, true, None));
        assert!(!re.exec(Some("".to_string()), false, false, None));

        assert!(!re.exec(Some("ab".to_string()), true, true, None));
        assert!(!re.exec(Some("ab".to_string()), true, false, None));
        assert!(!re.exec(Some("ab".to_string()), false, true, None));
        assert!(!re.exec(Some("ab".to_string()), false, false, None));

        assert!(!re.exec(Some("aab".to_string()), true, true, None));
        assert!(!re.exec(Some("aab".to_string()), true, false, None));
        assert!(!re.exec(Some("aab".to_string()), false, true, None));
        assert!(!re.exec(Some("aab".to_string()), false, false, None));
    }

    #[test]
    fn test_integration3() {
        let re = Regex::new("a+b+");

        assert!(re.exec(Some("abc".to_string()), true, true, None));
        assert!(re.exec(Some("abc".to_string()), true, false, None));
        assert!(re.exec(Some("abc".to_string()), false, true, None));
        assert!(!re.exec(Some("abc".to_string()), false, false, None));

        assert!(re.exec(Some("zabc".to_string()), true, true, None));
        assert!(!re.exec(Some("zabc".to_string()), true, false, None));
        assert!(re.exec(Some("zabc".to_string()), false, true, None));
        assert!(!re.exec(Some("zabc".to_string()), false, false, None));

        assert!(re.exec(Some("aaaaaaabbbbbbbc".to_string()), true, true, None));
        assert!(re.exec(Some("aaaaaaabbbbbbbc".to_string()), true, false, None));
        assert!(re.exec(Some("aaaaaaabbbbbbbc".to_string()), false, true, None));
        assert!(!re.exec(Some("aaaaaaabbbbbbbc".to_string()), false, false, None));
    }

    #[test]
    fn test_integration4() {
        {
            let re = Regex::new("(ab)c");

            assert!(re.exec(Some("zabcz".to_string()), true, true, None));
            assert!(!re.exec(Some("zabcz".to_string()), true, false, None));
            assert!(re.exec(Some("zabcz".to_string()), false, true, None));
            assert!(!re.exec(Some("zabcz".to_string()), false, false, None));

            assert!(!re.exec(Some("ac".to_string()), true, true, None));
            assert!(!re.exec(Some("ac".to_string()), true, false, None));
            assert!(!re.exec(Some("ac".to_string()), false, true, None));
            assert!(!re.exec(Some("ac".to_string()), false, false, None));
        }
        {
            let re = Regex::new("a|c");

            assert!(re.exec(Some("a".to_string()), true, true, None));
            assert!(re.exec(Some("a".to_string()), true, false, None));
            assert!(re.exec(Some("a".to_string()), false, true, None));
            assert!(re.exec(Some("a".to_string()), false, false, None));

            assert!(re.exec(Some("c".to_string()), true, true, None));
            assert!(re.exec(Some("c".to_string()), true, false, None));
            assert!(re.exec(Some("c".to_string()), false, true, None));
            assert!(re.exec(Some("c".to_string()), false, false, None));

            assert!(!re.exec(Some("b".to_string()), true, true, None));
            assert!(!re.exec(Some("b".to_string()), true, false, None));
            assert!(!re.exec(Some("b".to_string()), false, true, None));
            assert!(!re.exec(Some("b".to_string()), false, false, None));
        }
        {
            let re = Regex::new("(ab)*");

            assert!(re.exec(Some("ab".to_string()), true, true, None));
            assert!(re.exec(Some("ab".to_string()), true, false, None));
            assert!(re.exec(Some("ab".to_string()), false, true, None));
            assert!(re.exec(Some("ab".to_string()), false, false, None));

            assert!(re.exec(Some("ababab".to_string()), true, true, None));
            assert!(re.exec(Some("ababab".to_string()), true, false, None));
            assert!(re.exec(Some("ababab".to_string()), false, true, None));
            assert!(re.exec(Some("ababab".to_string()), false, false, None));

            assert!(re.exec(Some("aa".to_string()), true, true, None));
            assert!(re.exec(Some("aa".to_string()), true, false, None));
            assert!(re.exec(Some("aa".to_string()), false, true, None));
            assert!(!re.exec(Some("aa".to_string()), false, false, None));
        }
        {
            let re = Regex::new("(ac)|(bd)");

            assert!(re.exec(Some("ac".to_string()), true, true, None));
            assert!(re.exec(Some("ac".to_string()), true, false, None));
            assert!(re.exec(Some("ac".to_string()), false, true, None));
            assert!(re.exec(Some("ac".to_string()), false, false, None));

            assert!(re.exec(Some("bd".to_string()), true, true, None));
            assert!(re.exec(Some("bd".to_string()), true, false, None));
            assert!(re.exec(Some("bd".to_string()), false, true, None));
            assert!(re.exec(Some("bd".to_string()), false, false, None));

            assert!(!re.exec(Some("bc".to_string()), true, true, None));
            assert!(!re.exec(Some("bc".to_string()), true, false, None));
            assert!(!re.exec(Some("bc".to_string()), false, true, None));
            assert!(!re.exec(Some("bc".to_string()), false, false, None));
        }
        {
            let re = Regex::new("(ab)+");

            assert!(re.exec(Some("ab".to_string()), true, true, None));
            assert!(re.exec(Some("ab".to_string()), true, false, None));
            assert!(re.exec(Some("ab".to_string()), false, true, None));
            assert!(re.exec(Some("ab".to_string()), false, false, None));

            assert!(re.exec(Some("abababab".to_string()), true, true, None));
            assert!(re.exec(Some("abababab".to_string()), true, false, None));
            assert!(re.exec(Some("abababab".to_string()), false, true, None));
            assert!(re.exec(Some("abababab".to_string()), false, false, None));

            assert!(!re.exec(Some("a".to_string()), true, true, None));
            assert!(!re.exec(Some("a".to_string()), true, false, None));
            assert!(!re.exec(Some("a".to_string()), false, true, None));
            assert!(!re.exec(Some("a".to_string()), false, false, None));

            assert!(!re.exec(Some("".to_string()), true, true, None));
            assert!(!re.exec(Some("".to_string()), true, false, None));
            assert!(!re.exec(Some("".to_string()), false, true, None));
            assert!(!re.exec(Some("".to_string()), false, false, None));
        }
        {
            let re = Regex::new("(ab)?");

            assert!(re.exec(Some("ab".to_string()), true, true, None));
            assert!(re.exec(Some("ab".to_string()), true, false, None));
            assert!(re.exec(Some("ab".to_string()), false, true, None));
            assert!(re.exec(Some("ab".to_string()), false, false, None));

            assert!(re.exec(Some("abababab".to_string()), true, true, None));
            assert!(re.exec(Some("abababab".to_string()), true, false, None));
            assert!(re.exec(Some("abababab".to_string()), false, true, None));
            assert!(!re.exec(Some("abababab".to_string()), false, false, None));

            assert!(re.exec(Some("".to_string()), true, true, None));
            assert!(re.exec(Some("".to_string()), true, false, None));
            assert!(re.exec(Some("".to_string()), false, true, None));
            assert!(re.exec(Some("".to_string()), false, false, None));
        }
    }
}
