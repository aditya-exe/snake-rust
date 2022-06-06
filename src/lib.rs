mod random;

use std::collections::VecDeque;

use random::random_range;

pub type Position = (usize, usize);

#[derive(Debug)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug)]
pub struct SnakeGame {
    width: usize,
    height: usize,
    snake: VecDeque<Position>,
    direction: Direction,
    food: Position,
    finished: bool,
}

impl SnakeGame {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            width,
            height,
            snake: [((width - 2).max(0), (height / 2).max(0))]
                .into_iter()
                .collect(),
            direction: Direction::Left,
            food: (2.min(width - 1), height / 2),
            finished: false,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match (&self.direction, direction) {
            (Direction::Top, Direction::Top)
            | (Direction::Top, Direction::Bottom)
            | (Direction::Right, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Bottom, Direction::Top)
            | (Direction::Bottom, Direction::Bottom)
            | (Direction::Left, Direction::Right)
            | (Direction::Left, Direction::Left) => {}
            (_, direction) => self.direction = direction,
        }
    }

    pub fn is_valid(&self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }

    pub fn tick(&mut self) {
        if self.finished && self.snake.len() == 0 {
            return;
        }

        let (x, y) = self.snake[0];
        let new_head = match &self.direction {
            Direction::Top => (x, y - 1),
            Direction::Bottom => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        if !self.is_valid(new_head) || self.snake.contains(&new_head) {
            self.finished = true;
        } else {
            if new_head != self.food {
                self.snake.pop_back();
            } else {
                let free_positions = (0..self.height)
                    .flat_map(|y| (0..self.width).map(move |x| (x,y)))
                    .filter(|pos| !self.snake.contains(pos))
                    .collect::<Vec<_>>();
                
                if free_positions.is_empty() {
                    self.finished = true;
                    return;
                }
                self.food = free_positions[random_range(0, free_positions.len())];
            }
            self.snake.push_front(new_head);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SnakeGame;

    #[test]
    fn test() {
        println!("{:?}", SnakeGame::new(10, 10));
    }
}
