use chall83::c1::*;
use chall83::c2::*;
enum CONSONANTS  {B,C,D,F,G,H,J,K,L,M,N,P,Q,R,S,T,V,W,X,Z,}
//const CONSONANTS: char =  {B;C;D;F;G;H;J;K;L;M;N;P;Q;R;S;T;V;W;X;Z};
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
    println!("{}",piglatte(&argoz.content));
}
fn main() {
    c1();
}
pub struct Sentence {
 content: String,
}

impl Sentence {
    fn pigs(&self) -> &str {
        if cons(&self) == true {
            return 
        }
        }
    
    fn cons(&self) -> (bool) {
        let first: bool  = match &self.content.starts_with(CONSONANTS){
            Some(first) => Some((first)),
            None => None,
        };
    }

    fn vow(&self) -> (bool, char) {
        let first: bool  = match &self.content.starts_with(CONSONANTS){
            Some(first) => Some((first)),
            None => None,
        };
    fn revpop(&self) -> &str {
        let mut one: Vec<u8> = &self.content.split_whitespace().rev().collect();
        one.pop();
        one.iter().rev();
        one.push("fay");
        stringify!(one)
    };
    }
    }

