use chall83::c1::*;
use chall83::c2::*;
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
        match &self.consonant() {
            
        }
        }
    fn consonant(&self) -> Option<char> { 
        const CONSONANTS: &[char] =
         &['B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M',
            'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Z'];
        self.content.chars().next()
        .filter(|c| CONSONANTS.contains(c))
        //here i want to return the char that i want to replace, otherwise return None
    }
    fn vowel(&self) -> Option<char> {
        const VOWELS: &[char] =
         &['A', 'E', 'I', 'O', 'U'];
        self.content.chars().next()
        .filter(|c| VOWELS.contains(c))
    }
    fn conspop(&self, char) -> &str {
        let mut one: Vec<_> = self.content.split_whitespace().rev().collect();
        one.pop();
        one.iter().rev();
        one.push("fay");
        stringify!(one)
    }

    }

