#[allow(dead_code)]
#[derive(PartialEq, Clone)]
enum State {
    Unvisited,
    Visiting,
    Visited
}

#[allow(dead_code)]
pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut course_order = Vec::<i32>::with_capacity(num_courses as usize);

    let len = num_courses as usize;

    let mut node_states = vec![State::Unvisited; len];
    let mut adj_list = vec![Vec::<i32>::new(); len];

    for edge in prerequisites.iter() {
        adj_list[edge[0] as usize].push(edge[1]);
    }

    for i in 0..num_courses as usize {
        if node_states[i] == State::Unvisited
            && is_circle(i, &mut node_states, &adj_list, &mut course_order) {
            return Vec::new();
        }
    }

    course_order
}

#[allow(dead_code)]
fn is_circle(current: usize, node_state: &mut Vec<State>, adj_list: &Vec<Vec<i32>>, course_order: &mut Vec<i32>) -> bool {
    node_state[current] = State::Visiting;

    for &next in adj_list[current].iter() {
        let next_state = &node_state[next as usize];
        if *next_state == State::Visited {
            continue;
        }
        if *next_state == State::Visiting {
            return true;
        }
        if is_circle(next as usize, node_state, adj_list, course_order) {
            return true;
        }
    }

    node_state[current] = State::Visited;
    course_order.push(current as i32);
    false
}

#[cfg(test)]
mod class_schedule {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn fail() {
        assert_eq!(true, false);
    }
}
