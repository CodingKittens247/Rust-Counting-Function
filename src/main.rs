use std::collections::HashMap;

fn main() {
    let nums: [i32; 23] = [1,2,3,4,5,6,1,2,3,1,2,1,2,1,7,9,6,6,6,6,102,100,9001];
    //first we initialize what will be counted

    let mut list: HashMap<i32, i32> = HashMap::new();
    //we initialize what will be put to count, as we need to store the keys, in this case, 1 for example, and then the amount of times it appears therefore its a KVP
    //let count:u32 = nums.len().into() as u32;

    for count in nums {
        let count: &mut i32 = list.entry(count).or_insert(0);
        *count += 1;
        //this basically says count in the amount of entries in the nums array we defined earlier, it will parse into it, and see what is in there, andthe entry or insert (method from the book) will insert if it doesn't exist as a KVP, if it exists, it will give it another V in the K field
}
println!("{list:?}");
}
