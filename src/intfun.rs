use std::collections::HashMap;
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
     return end as f64
    }
   
   //this is the mode of the vector, so far ive tried to not make structs or specialtypes, since i dont want to yet, basically we make a simple for loop with a vector, to iterate over it 
    pub fn modeint(nums: &[i32]) -> HashMap<i32, i32> {
      use std::collections::HashMap;
      let nus = nums.to_vec();
      //why do we .to_vec and not clone? i dont want a reference lol 
      let mut numbs = HashMap::new();
      //why do we make a hashmap? to store K and V, as in, number and amount, as explained in the book, with this function, we insert the number, then the number of times it appears, so, the K is the number, and the V is the amount, only one K can exist, so if it does, it adds 1, otherwise the K gets added.
      for numbe in nus //this mf right here iterates over the elements in the vec we made earlier, with a new variable, so it can really be anythng as longas it pass it into the var below 
      {
         let count = numbs.entry(numbe).or_insert(0); // this is really important, and is the crutch of the logic, what it does basically, new variable, that variable, basically is the V in the KV, the hint is in the line below, it has a *count and increments? why would it do that? you ask, it basically takes the count (remember, the Key in the hashmap is the number, the V is the COUNT, AS IN HOW MANY IT HAS!, knowing that, it basically, sees if we have K V of 1, 5 (that is 1, 5 times), it will grab, dereference the value, and add another one, then it repeats, if it doesn't exist, it adds the value! how quaint!)
         *count += 1;
      };
      return numbs; // this fucin line just returns the hashmap, too lazy to test this properly so you're getting a :? and you'll be grateful
    }