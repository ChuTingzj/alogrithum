pub struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let s = s.as_str();
        let mut s: Vec<&str> = s.split("").collect();
        s.sort();
        let t = t.as_str();
        let mut t: Vec<&str> = t.split("").collect();
        t.sort();
        s.eq(&t)
    }
}
