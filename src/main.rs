use genesis_mobo::{balanced_check, count_recurring_elems};

fn main() {
    let s: Vec<&str> = vec!["{{{}}}", "{{}}(())", "(([]]][]]))", "", "}", "{"];

    println!("{:?} --> \n{:?}", &s, balanced_check(&s));

    let s: Vec<i32> = vec![1, 1, 2, 2, 3, 4, 3, 3];

    println!("{:?} --> \n{}", &s, count_recurring_elems(&s));
}
