
    pub fn medianint(nums: &[i32]) -> i32 {
    //my modus operandi here is gonna be, take the vectored list, parse it (order it) and met len it, and return the middle one if its a prime
    //that was my first approach, but i refactored it into taking a slice of the entire vector, so i could easierly modify it without much hassle.
               //let count: usize = nums.len(); // we need to know what the size of the vector is, so we can determine its length and if its div by 2
     let sorted: Vec<i32> = nums.to_vec(); // we make a new vec from the slice, so we dont take ownership and modify the og input
     let len = sorted.len();
     let end: i32 = if len % 2 == 0 
     {  
      sorted[len / 2]
        
     } else {
        (sorted[len / 2] + sorted[(len / 2) - 1]) / 2 //in this case, if its not even, we will just grab the 2 middle elements of the vector, and divide divide their value, that way we can kinda get the median lol 
     };
     return end
    }