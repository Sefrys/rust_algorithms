use std::collections::HashMap;

// Counting recurring elements

pub fn count_recurring_elems(vec: &Vec<i32>) -> i32 {
    let mut values = HashMap::new();

    vec.iter().for_each(|&x| {
        let count = values.entry(x).or_insert(0);
        *count += 1;
    });

    values.iter().filter(|&(_key, value)| *value > 1).count() as i32
}

// Bracket balancing

pub fn balanced_check(vec: &Vec<&str>) -> Vec<String> {
    vec.iter().map(|s| is_balanced(&s)).collect()
}

fn is_balanced(s: &str) -> String {
    let mut stack: Vec<char> = Vec::new();

    if s.len() <= 1 {
        return "NO".to_string();
    }

    for c in s.chars() {
        match c {
            '{' | '(' | '[' => stack.push(c),
            '}' | ')' | ']' => {
                if let Some(prev) = stack.pop() {
                    match (prev, c) {
                        ('{', '}') | ('(', ')') | ('[', ']') => continue,
                        _ => return "NO".to_string(),
                    }
                } else {
                    return "NO".to_string();
                }
            }
            _ => continue,
        }
    }

    "YES".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced() {
        assert_eq!("YES", is_balanced("{{[[(())]]}}"));
    }

    #[test]
    fn not_balanced() {
        assert_eq!("NO", is_balanced("}}{[{}}]}"));
    }

    #[test]
    fn single_opening() {
        assert_eq!("NO", is_balanced("{"));
    }

    #[test]
    fn single_closing() {
        assert_eq!("NO", is_balanced("}"));
    }
    #[test]
    fn empty_string() {
        assert_eq!("NO", is_balanced(""));
    }

    #[test]
    fn check_array() {
        let s: Vec<&str> = vec!["{{{}}}", "{{}}(())", "(([]]][]]))"];
        assert_eq!(vec!["YES", "YES", "NO"], balanced_check(&s));
    }

    #[test]
    fn empty_arr() {
        assert_eq!(0, count_recurring_elems(&vec![]));
    }

    #[test]
    fn two_non_unique() {
        assert_eq!(2, count_recurring_elems(&vec![1, 1, 2, 2]));
    }

    #[test]
    fn three_non_uniq_one_uniq() {
        assert_eq!(3, count_recurring_elems(&vec![3, 1, 4, 1, 2, 2, 4, 4]));
    }
}
