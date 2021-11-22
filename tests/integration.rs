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

    #[test]
    fn test_integration4() {
        {
            let re = Regex::new("(ab)c");

            assert_eq!(true, re.exec("zabcz", true, true));
            assert_eq!(false, re.exec("zabcz", true, false));
            assert_eq!(true, re.exec("zabcz", false, true));
            assert_eq!(false, re.exec("zabcz", false, false));

            assert_eq!(false, re.exec("ac", true, true));
            assert_eq!(false, re.exec("ac", true, false));
            assert_eq!(false, re.exec("ac", false, true));
            assert_eq!(false, re.exec("ac", false, false));
        }
        {
            let re = Regex::new("a|c");

            assert_eq!(true, re.exec("a", true, true));
            assert_eq!(true, re.exec("a", true, false));
            assert_eq!(true, re.exec("a", false, true));
            assert_eq!(true, re.exec("a", false, false));

            assert_eq!(true, re.exec("c", true, true));
            assert_eq!(true, re.exec("c", true, false));
            assert_eq!(true, re.exec("c", false, true));
            assert_eq!(true, re.exec("c", false, false));

            assert_eq!(false, re.exec("b", true, true));
            assert_eq!(false, re.exec("b", true, false));
            assert_eq!(false, re.exec("b", false, true));
            assert_eq!(false, re.exec("b", false, false));
        }
        {
            let re = Regex::new("(ab)*");

            assert_eq!(true, re.exec("ab", true, true));
            assert_eq!(true, re.exec("ab", true, false));
            assert_eq!(true, re.exec("ab", false, true));
            assert_eq!(true, re.exec("ab", false, false));

            assert_eq!(true, re.exec("ababab", true, true));
            assert_eq!(true, re.exec("ababab", true, false));
            assert_eq!(true, re.exec("ababab", false, true));
            assert_eq!(true, re.exec("ababab", false, false));

            assert_eq!(true, re.exec("aa", true, true));
            assert_eq!(true, re.exec("aa", true, false));
            assert_eq!(true, re.exec("aa", false, true));
            assert_eq!(false, re.exec("aa", false, false));
        }
        {
            let re = Regex::new("(ac)|(bd)");

            assert_eq!(true, re.exec("ac", true, true));
            assert_eq!(true, re.exec("ac", true, false));
            assert_eq!(true, re.exec("ac", false, true));
            assert_eq!(true, re.exec("ac", false, false));

            assert_eq!(true, re.exec("bd", true, true));
            assert_eq!(true, re.exec("bd", true, false));
            assert_eq!(true, re.exec("bd", false, true));
            assert_eq!(true, re.exec("bd", false, false));

            assert_eq!(false, re.exec("bc", true, true));
            assert_eq!(false, re.exec("bc", true, false));
            assert_eq!(false, re.exec("bc", false, true));
            assert_eq!(false, re.exec("bc", false, false));
        }
        {
            let re = Regex::new("(ab)+");

            assert_eq!(true, re.exec("ab", true, true));
            assert_eq!(true, re.exec("ab", true, false));

            // TODO: NFA型エンジンにおいて、PlusとQuestionの引数にAstTreeが入ってきてもインスタンス構築ができるようにする
            //assert_eq!(true, re.exec("ab", false, true));
            //assert_eq!(true, re.exec("ab", false, false));
        }
    }
}
