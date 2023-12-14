mod solution;
fn main() {
    let vec = Vec::from([3, 0, 1]);
    let result = solution::Solution::missing_number(vec);
    println!("{}", result)
}
