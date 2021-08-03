mod snake_test;

use std::time::Duration;
use crossterm::event::{read, poll, KeyCode, Event};
use rand::Rng;

struct SnakeNode {
    pos: (i32, i32),
}

pub struct Game {
    snake: Vec<SnakeNode>,
    food: (i32, i32),
    score: i32,
}

impl Game {

    fn new() -> Self {
        Game {
            snake: vec![],
            food: (-1, -1),
            score: 0,
        }
    }

    fn play(& mut self) -> Option<()> {

        Game::title();
        loop {
            println!("Would you like to play? (y/n)");
            
            let mut play_state = String::new();

            std::io::stdin()
                    .read_line(&mut play_state)
                    .expect("Error reading user input for play state!");

            play_state = play_state.trim().parse().expect("Error parsing input for play state!");
    
            if play_state.to_ascii_lowercase().eq(&String::from("y")) {
                let result = self.game_loop();

                if result {
                    println!("Congratulations, you win!");
                } else {
                    println!("You lose!");
                }

                break;
            }
            else if play_state.to_ascii_lowercase().eq(&String::from("n")) {
                println!("Goodbye!");
                return None;
            }
            else {
                println!("Please enter in one of 'y' or 'n' !");
                continue;
            }
        }
        
        Some(())
    }

    fn game_loop(&mut self) -> bool {


        self.snake.push(SnakeNode {
            pos: (7, 7)
        });

        self.set_food();

        loop {

            self.print_game();

            if self.snake.len() == 256 {
                return true;
            }

            for node in self.snake[1..].iter() {
                if Game::check_collisions(node.pos, self.snake[0].pos) {
                    return false;
                }
            }

            let result = match self.check_input() {
                Some(pos) => pos,
                None => return false
            };

            
            if Game::check_collisions(result, self.food) {
                self.snake.push(SnakeNode {
                    pos: (-1, -1),
                });
                
                self.score += 10;
                self.set_food();
            }
            
            self.update_snake(result);
            
        }
    }

    fn check_collisions(item1: (i32, i32), item2: (i32, i32)) -> bool {
        (item1.0, item1.1) == (item2.0, item2.1)
    }

    fn set_food(&mut self) {

        'outer: loop {
            let food_x = rand::thread_rng().gen_range(0..16);
            let food_y = rand::thread_rng().gen_range(0..16);

            if self.snake.len() != 0 {
                for node in self.snake.iter() {
                    if Game::check_collisions(node.pos, (food_x, food_y)) {
                        continue 'outer;
                    }
                }
            }

            self.food = (food_x, food_y);
            break;
        }
    }

    fn title() {
        println!("*******************************************");
        println!("
        _____             _        
       / ____|           | |       
      | (___  _ __   __ _| | _____ 
       \\___ \\| '_ \\ / _` | |/ / _ \\
       ____) | | | | (_| |   <  __/
      |_____/|_| |_|\\__,_|_|\\_\\___|
                                   
                ");
        println!("*******************************************\n");
    }
      
    fn clear_screen() {
        println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn print_game(&self) {

        Game::clear_screen();

        println!("------------------");

        let pos: Vec<(i32, i32)> = self.snake.iter().map(|node| node.pos).collect();

        for i in 0..16 {
            print!("|");
            for j in 0..16 {
                
                if pos.contains(&(j, i)) {
                    if (j, i) == self.snake[0].pos {
                        print!("o")
                    } else {
                        print!("+");
                    }
                } else if (j, i) == self.food {
                    print!(".");
                } else {
                    print!(" ");
                }
            }
            print!("|\n");
        }

        println!("------------------");

        println!("Current score: {}", self.score);
        println!("Head: {}, {}", self.snake[0].pos.0, self.snake[0].pos.1);
    }

    fn check_input(&self) -> Option<(i32, i32)> {
        loop {
            if poll(Duration::from_millis(100)).ok()? {

                match read().ok()? {
                    Event::Key(key) => {
                        match key.code {
                            KeyCode::Up => {
                                println!("Up!");
                                return Some((self.snake[0].pos.0, self.snake[0].pos.1 - 1));
                            },
                            KeyCode::Left => {
                                println!("Left!");
                                return Some((self.snake[0].pos.0 - 1, self.snake[0].pos.1));
                            },
                            KeyCode::Right => {
                                println!("Right!");
                                return Some((self.snake[0].pos.0 + 1, self.snake[0].pos.1));
                            },
                            KeyCode::Down => {
                                println!("Down!");
                                return Some((self.snake[0].pos.0, self.snake[0].pos.1 + 1));
                            },
                            KeyCode::Esc => {
                                return None;
                            }
                            _ => continue,
                        }
                    },
                    _ => continue,
                }
            }
        }
    }

    fn update_snake(&mut self, new_pos: (i32, i32)) -> bool {

        if self.snake.len() == 0 {
            return false;
        } else if (new_pos.0 > 15) || (new_pos.1 > 15)
            || (new_pos.0 < 0 ) || (new_pos.1 < 0) {
            return false;
        }

        let mut next = self.snake[0].pos;
        self.snake[0].pos = new_pos;

        for node in self.snake[1..].iter_mut() {
            let temp = node.pos;
            node.pos = next;
            next = temp;
        }
        
        true
    }
}

fn main() {
    let mut game = Game::new();
    game.play();
}
