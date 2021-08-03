#[cfg(test)]
mod test {
    
    use crate::SnakeNode;
    use crate::Game;

    #[test]
    fn update_snake_test() {

        let s = SnakeNode {
            pos: (7, 7)
        };

        let a = SnakeNode {
            pos: (-1, -1)
        };

        let b = SnakeNode {
            pos: (-1, -1)
        };

        let mut g = Game {
            snake: vec![],
            food: (-1, -1),
            score: -1,
        };

        g.snake.push(s);
        g.snake.push(a);

        // g.update_snake((17,7));

        // assert_eq!((g.snake[0].pos.0, g.snake[0].pos.1), (7, 7));
        // assert_eq!((g.snake[1].pos.0, g.snake[1].pos.1), (-1, -1));

        // g.update_snake((-2,7));

        // assert_eq!((g.snake[0].pos.0, g.snake[0].pos.1), (7, 7));
        // assert_eq!((g.snake[1].pos.0, g.snake[1].pos.1), (-1, -1));

        g.update_snake((5, 4));

        assert_eq!((g.snake[0].pos.0, g.snake[0].pos.1), (5, 4));
        assert_eq!((g.snake[1].pos.0, g.snake[1].pos.1), (7, 7));

        g.snake.push(b);

        g.update_snake((6, 3));
        assert_eq!((g.snake[0].pos.0, g.snake[0].pos.1), (6, 3));
        assert_eq!((g.snake[1].pos.0, g.snake[1].pos.1), (5, 4));
        assert_eq!((g.snake[2].pos.0, g.snake[2].pos.1), (7, 7));
    }

    #[test]
    fn test_input() {
        let mut g = Game {
            snake: vec![],
            food: (-1, -1),
            score: -1,
        };

        let s = SnakeNode {
            pos: (7, 7)
        };

        g.snake.push(s);

        g.check_input().unwrap();
    }
}