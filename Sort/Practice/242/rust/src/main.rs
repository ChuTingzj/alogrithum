mod solution;
fn main() {
    let result = solution::Solution::is_anagram(String::from("abc"), String::from("bca"));
    println!("{}", result)
}
