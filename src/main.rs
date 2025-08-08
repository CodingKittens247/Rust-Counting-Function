//use std::collections::HashMap;
use chall83::intfun::medianint;
use chall83::intfun::modeint;
fn main() {
    let nums: Vec<i32> = vec![20,21,301,1024,681,20];
    println!("the number in the middle when sorted is {}",medianint(&nums));
    println!("the number that shows up the most is {}",modeint(&nums));
}
