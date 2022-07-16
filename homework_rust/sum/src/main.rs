
pub fn sum(values: &[u32]) -> u32 {
    values.iter().sum()
}


fn main() {
    let values: Vec<u32> = vec!(100,150,200,250);
    let s = Option::Some(sum(&values));
    println!("sum is {:?}", s);
}
