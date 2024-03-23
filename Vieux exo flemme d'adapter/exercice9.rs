use std::collections::VecDeque;

use itertools::max;

pub fn compute_results() -> (u32, u32) {
    let last_marble: usize = 70953;
    let number_of_players: usize = 405;
    dbg!(format!("{}", i32::rem_euclid(-1, 3)));
    return (solve(&last_marble, &number_of_players), solve(&(last_marble*100), &number_of_players));
}



fn solve(last_marble: &usize, number_of_players: &usize) -> u32 {
    let mut scores = vec![0; *number_of_players];
    let mut circle = VecDeque::new();
    circle.push_front(0);

    for marble_to_add in 1..last_marble+1 {
        if marble_to_add % 23 == 0 {
            (0..7).for_each(|_| {
                let current = circle.pop_back().expect("Rotate problem"); 
                circle.push_front(current);
            });
            scores[marble_to_add%number_of_players] += marble_to_add + circle.pop_front().unwrap();

        } else {
            (0..2).for_each(|_| { 
                let tmp = circle.pop_front().expect("Rotate problem");        
                circle.push_back(tmp);
            });
            circle.push_front(marble_to_add);
        }
    }

    return max(scores).unwrap() as u32;
}

