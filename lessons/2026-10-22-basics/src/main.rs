fn main() {
    let mut position = 0;
    let mut speed = 2;
    let mut final_step: i32 = 0;

    for step in 1..=10 {
        speed += 2;
        if speed > 8 {
            speed = 8;
        }

        position += speed;
        println!("Step {}, Speed {}, Position {}", step, speed, position);

        if position >= 25 {
            println!("Destination reached!");
            final_step = step;
            break;
        }
    }

    println!("Final step {}, final position {}", final_step, position);
    println!("Simulation finished");
}
