/*
 Program to test three-door problem.
 This problem is based on how the host choose the damn door.

 Assume I choose door 0, and the displayed door is door 1.
 Let event A means that door 0 has the car.
 Let event B means that door 1 has a goat.
 Then we need the probability of event A|B
 P(A|B) = P(AB) / P(B) = (1/3) / (2/3) = 1/2
 This implementation is based on the assumption that
    the host should never know whether the displayed door has a goat until he opens it.

 Another implementation may be based on the assumption that
    the host will always choose to open a door that has a goat.
    In this case the probability will be 2/3.

 The conflict is because if the host has no idea whether the door will have a car or a goat
    before opening it, then this is totally a conditional probability problem,
    otherwise the door left will has the initial probability 2/3 which the unselected doors hold.
 */

//use std::io;
//use std::cmp::Ordering;
use rand::Rng;

const ALWAYS_PICK_GOAT: bool = false;

const TOT: i32 = 1000000;
const CHANGE: bool = true;

fn main() {
    let mut cnt = 0;
    let mut win = 0;
    while cnt < TOT {
        let secret = rand::thread_rng().gen_range(0, 3);
        let mut choose = 0;
        let mut display = 1;
        if secret == display {
            if ALWAYS_PICK_GOAT {
                display = 2;
            } else {
                continue;
            }
        }
        cnt += 1;
        let left = 3 - choose - display;
        if CHANGE {
            choose = left;
        }
        if choose == secret {
            win += 1;
        }
    }
    println!("Win rate is {}", win as f64 / TOT as f64);
}

