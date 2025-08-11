//this is all the functions required for the first part of the challange, ergo c1
    pub fn medianint(nums: &[i32]) -> f64 {
    //my modus operandi here is gonna be, take the vectored list, parse it (order it) and met len it, and return the middle one if its a prime
    //that was my first approach, but i refactored it into taking a slice of the entire vector, so i could easierly modify it without much hassle.
     let sorted: Vec<i32> = nums.to_vec(); // we make a new vec from the slice, so we dont take ownership and modify the og input
     let len: usize = sorted.len();
     let end: f64 = if len % 2 == 1 
     {  
      sorted[len / 2] as f64 //we grab the copied vec, and index half the len, t find half if it is divissible by 2
        
     } else {
        (sorted[len / 2] + sorted[(len / 2) - 1]) as f64 / 2.0 //in this case, if its not even, we will just grab the 2 middle elements of the vector, and divide divide their value, that way we can kinda get the median lol 
     };
     return end
    }
   
   //this is the mode of the vector, so far ive tried to not make structs or specialtypes, since i dont want to yet, basically we make a simple for loop with a vector, to iterate over it 
    pub fn modeint(nums: &[i32]) -> i32 {
      use std::collections::HashMap;
      let mut numbs: HashMap<i32, i32> = HashMap::new();
      //why do we make a hashmap? to store K and V, as in, number and amount, as explained in the book, with this function, we insert the number, then the number of times it appears, so, the K is the number, and the V is the amount, only one K can exist, so if it does, it adds 1, otherwise the K gets added.
      for &numbe in nums //here we changed from the previous commit, i didnt need a cloned vec or anything, since its a slice already i didnt need to, might refactor the fn above but really dont care lol 
            {
         let count = numbs.entry(numbe).or_insert(0); // this is really important, and is the crutch of the logic, what it does basically, new variable, that variable, basically is the V in the KV, the hint is in the line below, it has a *count and increments? why would it do that? you ask, it basically takes the count (remember, the Key in the hashmap is the number, the V is the COUNT, AS IN HOW MANY IT HAS!, knowing that, it basically, sees if we have K V of 1, 5 (that is 1, 5 times), it will grab, dereference the value, and add another one, then it repeats, if it doesn't exist, it adds the value! how quaint!)
         *count += 1;
      };
      numbs.into_iter().max_by_key(|&(_, count)| count).map(|(num, _)| num).unwrap() //i still dont fully understand how to filter hashmaps, and return, why i kinda did a hashmap, then turned the result ig into an option and unwrapped the option into a ptype, i tried so many diff ways without exhausting lifetime rules and borrowing, references etc, but it was really hard
}
