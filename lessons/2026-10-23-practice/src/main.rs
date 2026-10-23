fn main() {
    // Step 1: change speed to 3. Predict the output before running.
    let speed = 5;
    let mut position = 0;

    for step in 1..=3 {
        position += speed;
        println!("Step {}, Speed {}, Position {}", step, speed, position);
    }

    println!("Simulation finished");
}
