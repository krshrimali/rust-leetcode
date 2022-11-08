pub fn is_valid(inp: String) -> bool {
    let mut paren_stack: Vec<char> = vec![];
    let open_brackets: Vec<char> = vec!['{', '[', '('];
    let close_brackets: Vec<char> = vec!['}', ']', ')'];

    // Enumerating just in case I need index (_idx) later...
    for (_idx, inp_char) in inp.chars().enumerate() {
        if open_brackets.contains(&inp_char) {
            paren_stack.push(inp_char);
        } else if close_brackets.contains(&inp_char) {
            if paren_stack.is_empty() {
                return false;
            } else {
                let output = match paren_stack[paren_stack.len() - 1] {
                    '{' => inp_char == '}',
                    '[' => inp_char == ']',
                    '(' => inp_char == ')',
                    _ => false,
                };

                if !output {
                    return false;
                }

                paren_stack.pop();
            }
        }
    }
    paren_stack.is_empty()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(is_valid("{}{}{}()[]".to_string()));
    }

    #[test]
    fn ex2() {
        assert!(is_valid("()[]{}".to_string()));
    }

    #[test]
    fn ex3() {
        assert!(!is_valid("({}[)".to_string()));
    }

    #[test]
    fn ex4() {
        assert!(!is_valid("(".to_string()));
    }

    #[test]
    fn ex5() {
        assert!(!is_valid("((}}".to_string()));
    }

    #[test]
    fn ex6() {
        assert!(!is_valid("}(){}".to_string()))
    }
}
