use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    style::Print,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use rand::Rng;

const WIDTH: u16 = 20;
const HEIGHT: u16 = 10;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct SnakeGame {
    snake: Vec<(u16, u16)>, // Vector of coordinates
    apple: (u16, u16),
    direction: Direction,
    game_over: bool,
}

impl SnakeGame {
    fn new() -> Self {
        let snake = vec![(10, 5)];
        let apple = SnakeGame::generate_apple(&snake);
        Self {
            snake,
            apple,
            direction: Direction::Right,
            game_over: false,
        }
    }

    fn generate_apple(snake: &[(u16, u16)]) -> (u16, u16) {
        let mut rng = rand::thread_rng();
        loop {
            let pos = (rng.gen_range(0..WIDTH), rng.gen_range(0..HEIGHT));
            if !snake.contains(&pos) {
                return pos;
            }
        }
    }

    fn update(&mut self) {
        let mut new_head = self.snake[0];
        match self.direction {
            Direction::Up => new_head.1 = new_head.1.saturating_sub(1),
            Direction::Down => new_head.1 += 1,
            Direction::Left => new_head.0 = new_head.0.saturating_sub(1),
            Direction::Right => new_head.0 += 1,
        }

        // Check for collision with wall or self
        if new_head.0 >= WIDTH || new_head.1 >= HEIGHT || self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        self.snake.insert(0, new_head);

        if new_head == self.apple {
            self.apple = SnakeGame::generate_apple(&self.snake);
        } else {
            self.snake.pop();
        }
    }

    fn draw(&self) {
        let mut stdout = stdout();
        stdout.execute(terminal::Clear(ClearType::All)).unwrap();

        // Draw top border
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        print!("+");
        for _ in 0..WIDTH {
            print!("--");
        }
        print!("+");

        // Draw game area
        for y in 0..HEIGHT {
            stdout.execute(cursor::MoveTo(0, y + 1)).unwrap();
            print!("|");
            for x in 0..WIDTH {
                if self.snake.contains(&(x, y)) {
                    print!("██");
                } else if self.apple == (x, y) {
                    print!("@@");
                } else {
                    print!("  ");
                }
            }
            print!("|");
        }

        // Draw bottom border
        stdout.execute(cursor::MoveTo(0, HEIGHT + 1)).unwrap();
        print!("+");
        for _ in 0..WIDTH {
            print!("--");
        }
        print!("+");

        stdout.flush().unwrap();
    }

    fn change_direction(&mut self, new_dir: Direction) {
        use Direction::*;
        if (self.direction == Up && new_dir != Down)
            || (self.direction == Down && new_dir != Up)
            || (self.direction == Left && new_dir != Right)
            || (self.direction == Right && new_dir != Left)
        {
            self.direction = new_dir;
        }
    }
}

fn main() {
    let mut game = SnakeGame::new();
    terminal::enable_raw_mode().unwrap();

    let mut last_frame = Instant::now();

    while !game.game_over {
        if event::poll(Duration::from_millis(50)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Up => game.change_direction(Direction::Up),
                    KeyCode::Down => game.change_direction(Direction::Down),
                    KeyCode::Left => game.change_direction(Direction::Left),
                    KeyCode::Right => game.change_direction(Direction::Right),
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        if last_frame.elapsed() >= Duration::from_millis(150) {
            game.update();
            game.draw();
            last_frame = Instant::now();
        }
    }

    println!("\nGame Over!");
    terminal::disable_raw_mode().unwrap();
}
