use chall83::c1::*;
//use chall83::c2::*;
//enum CONSONANTS  {B,C,D,F,G,H,J,K,L,M,N,P,Q,R,S,T,V,W,X,Z,}
// enum Charac {
// }
fn c1() {
    let nums: Vec<i32> = vec![20,21,301,1024,681,20];
    println!("the number in the middle when sorted is {}",medianint(&nums));
    println!("the number that shows up the most is {}",modeint(&nums));
}
fn c2(){
    let arg = String::from("First");
    let argoz = Sentence {
        content: arg,
    };
    println!("{:?}",argoz.piglatino()); //:? because no pretty print 
}
fn main() {
    c1();
}
pub struct Sentence {
 content: String,
}
//i want a function that isolates the char to remove/modify, for consonants, another for  vowels, etc 
impl Sentence {
    fn piglatino(&self) -> Option<&str> {
        {
        }}
        
    fn consonant(&self) -> Option<char> { //why do i return option? because i like it, i will not do the check if it ishere, because i am lazy and dont plan ahead, and the boiletplate part is already done, i should really learn to plan my structures better!
        const CONSONANTS: &[char] =
         &['B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M',
            'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Z'];
        self.content.chars().next()
        .filter(|c| CONSONANTS.contains(c))
        //here i want to return the char that i want to replace, otherwise return None -- if we are using slices of strings, why return char? because its easier for me to find it if i convert it to char lol
    }
    fn vowel(&self) -> Option<char> { // same function as above but with diff criteria because i copy pasted, i should refsctor this shyte
        const VOWELS: &[char] =
         &['A', 'E', 'I', 'O', 'U'];
        self.content.chars().next()
        .filter(|c| VOWELS.contains(c))
    }
    fn conspop(&self) -> &str {
        let mut one: Vec<_> = self.content.split_whitespace().rev().collect();
        one.pop();
        one.iter().rev();
        one.push(self.consonant().unwrap().to_string().as_str());
        stringify!(one)
    }//holy batman what the hell happened here? u ask, well you see, strings are just vecs of u8, so i converted it to its most primal form, then i reversed it,removed the most element to the right (last), then i pushed the char (UNWRAPPED FOR OPTION, TO STRING FOR UTF 8 AND THEN TO STR BECAUSE SLICES ARE BEST! for this, then i stringify the spaghetti and return it in one)

    }


