type TimerCounts = [usize; 9];

fn main() {
    //let timers: Vec<usize> = include_str!("../example_input.txt")
    let timers: Vec<usize> = include_str!("../input.txt")
        .lines()
        .flat_map(|s| s.split(','))
        .map(|s| s.parse().unwrap())
        .collect();

    let mut timer_counts: TimerCounts = [0; 9];
    for timer in timers {
        timer_counts[timer] += 1;
    }

    for _ in 0..80 {
        timer_counts = tick(timer_counts);
    }

    let sum: usize = timer_counts.iter().sum();
    println!("After 80 days there are {} lanternfish", sum);

    for _ in 80..256 {
        timer_counts = tick(timer_counts);
    }

    let sum: usize = timer_counts.iter().sum();
    println!("After 256 days there are {} lanternfish", sum);
}

fn tick(timers: TimerCounts) -> TimerCounts {
    let mut next_count: TimerCounts = [0; 9];
    for (timer, timer_count) in timers.iter().enumerate() {
        if timer == 0 {
            next_count[6] += timer_count;
            next_count[8] += timer_count;
        } else {
            next_count[timer - 1] += timer_count;
        }
    }

    next_count
}
