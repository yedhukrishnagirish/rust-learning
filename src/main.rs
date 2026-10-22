fn main() {
    let mut position = 0;
    let mut speed = 2;

    let Steps : i32;
    for steps in 1..=10 {

        speed += 2;
        
        if speed>8 {
            speed = 8;
            println!("END of the pos");
        }

        position += speed;

        if position>=25 {
            println!("Destination Reached !!!");
            Steps = steps;
            break;
        }

    }

    println!(" Stpes {}, Speed {} , Position {} , Simulation finished", Steps, speed, position);
    
}
