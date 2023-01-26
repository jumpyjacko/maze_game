
const BOARD_SIZE: usize = 8;

type Point = (u32, u32);
type Maze = [[usize; BOARD_SIZE]; BOARD_SIZE];

enum Direction {
    Right,
    Left,
    Down,
    Up
}

#[derive(Debug)]
struct Entity {
    name: String,
    position: Point,
}

#[derive(Debug)]
struct State {
    maze: Maze,
    player: Entity,
    // If wanted, can add an enemy for theseus and the minotaur
}

const TILES: [&str; 3] = ["..", "##", "@."];

impl Entity {
    fn new(name: String, position: Point) -> Entity {
        Entity {
            name,
            position
        }
    }

    fn move_entity(self, position: Direction) -> Entity {
        match position {
            Direction::Right => todo!(),
            Direction::Left => todo!(),
            Direction::Down => todo!(),
            Direction::Up => todo!(),
        }
        
        todo!();
    }
}

impl State {
    fn new(maze: Maze, spawn: Point) -> State {
        State {
            maze,
            player: Entity::new("Player".to_owned(), spawn),
        }
    }

    fn update(self) -> State {
        todo!();
    }

    fn render(self) {
        /*
        Tileset will probably be,
           0 = .  | 1 = ## | 2 = @  | 3 = M  
           - read from State/self
           - use numbers in State/self to index the tileset
           - render & profit!
         */

        self.maze.iter().for_each(|row| {
            row.iter().for_each(|c| {
                print!("{}", TILES[*c]);
            });
            println!();
        });
    }

    fn check_win_state(self) -> bool {
        todo!();
    }
}

fn main() {
    // TODO: Procedurally generate a maze using the Depth-First Search (DFS) method to make 
    //       an adjacent neighbours table to then translate into a 2D array like this one
    let maze: Maze = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 1, 0, 1, 0, 1, 1],
        [1, 0, 0, 0, 1, 0, 0, 1],
        [1, 0, 1, 0, 1, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 1, 1, 0, 1],
        [1, 0, 1, 1, 1, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let state = State::new(maze, (1, 1));

    println!("{:?}", state);

    state.render();
}
