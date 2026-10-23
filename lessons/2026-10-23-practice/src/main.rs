fn limit_speed(speed: i32) -> i32 {
    if speed>8{
        return 8;
    }else {
        return speed;
    }
}

fn move_particle(position: i32, speed: i32) -> i32{
   return position + speed;
}

fn main() {
    let mut position = 0;
    let mut speed = 2;
    for steps in 1..=10 {
        speed = speed + 2;
        speed = limit_speed(speed);
        position = move_particle(position, speed);
    

        println!(" STEPS: {}, SPEED: {}, POSITION: {}", steps, speed, position);

        if position>=25{
            println!("Destination reached");
            break;
        }
    }
    println!("Simulation finished");
}