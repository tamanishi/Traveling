use std::io;

fn main() {
    let mut line1 = String::new();
    let _ = io::stdin().read_line(&mut line1);
    let step_cnt: u32 = line1.trim().parse().unwrap();

    let mut steps: Vec<Vec<i32>> = vec![];

    for _ in 0..step_cnt {
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        let step: Vec<i32> = line
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|e| e.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        steps.push(step);
    }

    let mut current: Vec<i32> = vec![0, 0];
    let mut prev_step = 0;

    for i in 0..steps.len() {
        let step = steps[i][0] - prev_step;
        prev_step = steps[i][0];
        let target = vec![steps[i][1], steps[i][2]];
        if can_move(&current, &target, step) {
            current[0] = target[0];
            current[1] = target[1];
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

fn can_move(current: &Vec<i32>, target: &Vec<i32>, step: i32) -> bool {
    let delta = (target[0] - current[0]).abs() + (target[1] - current[1]).abs();
    // println!("{} {}", delta, step);
    delta == step || (step > delta && ((step - delta) % 2 == 0))
}
