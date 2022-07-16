
enum State {
    Green,
    Yellow,
    Red,
}

pub trait Light {
    fn green_time(&self)->u32;
    fn yellow_time(&self)->u32;
    fn red_time(&self)->u32;
}

impl  Light for State {
   fn green_time(&self)->u32{
        60
   }
   fn yellow_time(&self)->u32{
        10
   }
   fn red_time(&self)->u32{
        120
   }
}

fn main() {
    let green = State::green_time(&State::Green);
    println!("green time is {:?}", green);
    let yellow = State::yellow_time(&State::Yellow);
    println!("yellow time is {:?}", yellow);
    let red = State::red_time(&State::Red);
    println!("red time is {:?}", red);
}
