pub struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_vec = unsafe { s.as_mut_vec() };
        s_vec.sort();
        let mut t_vec = unsafe { t.as_mut_vec() };
        t_vec.sort();
        s_vec.eq_ignore_ascii_case(&t_vec)
    }
}
