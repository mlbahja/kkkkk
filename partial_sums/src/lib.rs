pub fn parts_sums(arr: &[u64]) -> Vec<u64>{
    let mut res : Vec<u64> = Vec::new();
    let mut sum : u64 = 0;
    for n in arr {
        sum = sum + n;
        res.push(sum);
    }
    res.reverse();
    res.push(0);
    res
}