use std::ops::Add;
use std::collections::HashMap;

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn distance(self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Copy)]
struct Cursor {
    index: u32,
    coord: Coordinate,
    direction_index: usize,
    side_length: u32,
    side_offset: u32
}

static DIRECTIONS: &'static [Coordinate] = &[
    Coordinate {x: 0, y: 1}, 
    Coordinate {x: -1, y: 0}, 
    Coordinate {x: 0, y: -1}, 
    Coordinate {x: 1, y: 0}
];

static NEIGHBOURS: &'static [Coordinate] = &[
    Coordinate {x: -1, y:  1}, Coordinate {x:  0, y:  1}, Coordinate {x:  1, y:  1},  
    Coordinate {x: -1, y:  0},                            Coordinate {x:  1, y:  0},  
    Coordinate {x: -1, y: -1}, Coordinate {x:  0, y: -1}, Coordinate {x:  1, y: -1},
];

fn advance(current:Cursor) -> Cursor {
    if current.side_offset == current.side_length {
        if current.direction_index < 3 {
            Cursor {
                index: current.index + 1,
                coord: current.coord + DIRECTIONS[current.direction_index+1],
                direction_index: current.direction_index + 1,
                side_length: current.side_length,
                side_offset: 2
            }
        } else {
            Cursor {
                index: current.index + 1,
                coord: current.coord + Coordinate { x: 1, y: 0},
                direction_index: 0,
                side_length: current.side_length + 2,
                side_offset: 2 
            }
        }
    } else {
        Cursor { 
            index: current.index + 1,
            coord: current.coord + DIRECTIONS[current.direction_index],
            direction_index: current.direction_index,
            side_length: current.side_length,
            side_offset: current.side_offset + 1
        }
    }
}

fn main() {
    let input = 347991;
    
    let starting_cursor = Cursor { 
        index: 2, 
        coord: Coordinate { x: 1, y: 0 },
        side_length: 3,
        side_offset: 2,
        direction_index: 0,
    };

    let mut cursor = starting_cursor;

    

    while cursor.index < input{
        cursor = advance(cursor)
    }

    println!("Day 3a: {}", cursor.coord.distance());

    let mut values = HashMap::new();
    values.insert(Coordinate {x:0, y:0}, 1u32);
    let mut cursor = starting_cursor;
    let sum = loop {
        
        let mut sum = 0u32;
        for dir in NEIGHBOURS {
            let neighbour_coordinate = cursor.coord + *dir;
            sum += match values.get(&neighbour_coordinate) {
                Some(value) => *value,
                None => 0u32
            };
        }

        values.insert(cursor.coord, sum);

        if sum > input {
            break sum
        }

        cursor = advance(cursor)
    };

    println!("Day 3b: {}", sum);
}