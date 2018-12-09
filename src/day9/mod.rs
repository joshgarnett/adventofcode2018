struct Node {
    value: usize,
    next: usize,
    previous: usize,
}

struct CircularLinkedList {
    free: Vec<usize>,
    used: Vec<Node>,
    current_index: usize,
}

impl CircularLinkedList {
    fn new(initial: usize, capacity: usize) -> CircularLinkedList {
        let mut values = Vec::with_capacity(capacity);
        values.push(Node {
            value: initial,
            previous: 0,
            next: 0,
        });
        CircularLinkedList {
            free: Vec::new(),
            used: values,
            current_index: 0,
        }
    }

    fn get_free_node(&mut self) -> usize {
        match self.free.pop() {
            None => {
                // no free nodes, add one
                let index = &self.used.len();
                self.used.push(Node {
                    value: 0,
                    previous: 0,
                    next: 0,
                });
                *index
            }
            Some(i) => {
                i
            }
        }
    }

    fn insert(&mut self, value: usize) {
        let current_index = self.current_index;
        let old_next = self.used[current_index].next;

        let new_index = self.get_free_node();

        // update previous node to point to new node
        self.used[current_index].next = new_index;

        // add new node
        self.used[new_index].previous = current_index;
        self.used[new_index].next = old_next;
        self.used[new_index].value = value;

        // update next node to point to new node
        self.used[old_next].previous = new_index;

        self.current_index = new_index;
    }

    fn remove(&mut self) -> usize {
        let current_index = self.current_index;
        let previous = self.used[current_index].previous;
        let next = self.used[current_index].next;
        let value = self.used[current_index].value;
        self.used[previous].next = next;
        self.used[next].previous = previous;
        self.free.push(current_index);

        self.current_index = next;

        value
    }

    fn forward(&mut self, count: usize) {
        for _ in 0..count {
            self.current_index = self.used[self.current_index].next;
        }
    }

    fn back(&mut self, count: usize) {
        for _ in 0..count {
            self.current_index = self.used[self.current_index].previous;
        }
    }
}

pub fn part1() -> usize {
    calculate_score_fast(459, 71790)
}

pub fn part2() -> usize {
    calculate_score_fast(459, 71790 * 100)
}

#[allow(dead_code)]
fn calculate_score(players: usize, last_marble: usize) -> usize {
    let mut player_scores: Vec<usize> = vec![0; players];
    let mut board: Vec<usize> = Vec::new();

    board.push(0);
    let mut current_index: usize = 0;

    for m in 1..=last_marble {
        if m % 23 == 0 {
            current_index = (board.len() + current_index - 7) % board.len();
            player_scores[(m - 1) % players] += m + board.remove(current_index);
        } else {
            current_index = (current_index + 2) % board.len();
            board.insert(current_index, m);
        }
    }

    *player_scores.iter().max().unwrap()
}

fn calculate_score_fast(players: usize, last_marble: usize) -> usize {
    let mut player_scores: Vec<usize> = vec![0; players];
    let mut board = CircularLinkedList::new(0, last_marble);

    for m in 1..=last_marble {
        if m % 23 == 0 {
            board.back(7);
            player_scores[(m - 1) % players] += m + board.remove();
        } else {
            board.forward(1);
            board.insert(m);
        }
    }

    *player_scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(|| part1());
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        b.iter(|| part2());
    }

    #[test]
    fn part1_example1() {
        assert_eq!(32, calculate_score_fast(9, 25));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(8317, calculate_score_fast(10, 1618));
    }

    #[test]
    fn part1_example3() {
        assert_eq!(146373, calculate_score_fast(13, 7999));
    }

    #[test]
    fn part1_example4() {
        assert_eq!(2764, calculate_score_fast(17, 1104));
    }

    #[test]
    fn part1_example5() {
        assert_eq!(54718, calculate_score_fast(21, 6111));
    }

    #[test]
    fn part1_example6() {
        assert_eq!(37305, calculate_score_fast(30, 5807));
    }
}
