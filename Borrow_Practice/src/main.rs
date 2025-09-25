fn concat_strings(s1: &String, s2: &String) -> String {
    format!("{}{}", s1, s2)
}


fn clone_and_modify(s: &String) -> String {
    let mut cloned = s.clone();
    cloned.push_str("World!");
    cloned
}


#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    *total = 0;
    for i in low..=high{
        *total += i;
    }
}


fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"


    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"


    // create necessary variables and test your function for low 0 high 100
    // total should be 5050
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Total: {}", total)
}
