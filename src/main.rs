//use std::collections::HashMap;
use chall83::intfun::medianint;
use chall83::intfun::modeint;
fn main() {
    let nums: Vec<i32> = vec![1,2,3,4];
    println!("{}",medianint(&nums));
    println!("{:?}",modeint(&nums));
}
