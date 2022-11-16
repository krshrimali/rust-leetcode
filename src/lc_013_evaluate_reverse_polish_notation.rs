pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let stack: Vec<i32> = Vec::new();
    for token in tokens.iter() {
        if 
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1_evaluate_reverse_polish_notation() {
        let inp: Vec<String> = vec!["4".into(), "13".into(), "5".into(), "/".into(), "+".into()];
        assert_eq!(eval_rpn(inp), 6);
    }

    #[test]
    fn ex2_evaluate_reverse_polish_notation() {
        assert_eq!(
            eval_rpn(vec![
                "2".into(),
                "1".into(),
                "+".into(),
                "3".into(),
                "*".into()
            ]),
            9
        );
    }

    #[test]
    fn ex3_evaluate_reverse_polish_notation() {
        assert_eq!(eval_rpn(vec!["3".into(), "4".into(), "/".into()]), 0);
    }
}
