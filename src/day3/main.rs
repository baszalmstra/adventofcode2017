type LoopInfo = (u32, u32, u32);

fn find_loop_info(x:u32) -> LoopInfo {
    let mut loop_index:u32 = 0;
    let mut counter = 1;
    if x == 1 {
        return (0,1,1)
    }
    return loop {
        let count_in_loop = (loop_index*2+1)*4 + 4;
        counter += count_in_loop;
        if counter >= x {
            break (loop_index+1, counter - count_in_loop + 1,count_in_loop);
        }
        loop_index = loop_index + 1;
    };
}

fn find_position(x:u32) -> (i32, i32) {
    let (loop_index, starting_value, _count_in_loop) = find_loop_info(x);
    let (starting_position_x, starting_position_y) = (loop_index as i32, -(loop_index as i32)+1);

    if starting_value == x {
        return (starting_position_x, starting_position_y);
    };

    let side_length = (loop_index-1)*2 + 1;

    // Bottom side?
    let bottom_left_value = starting_value + side_length*3+2;
    if bottom_left_value <= x {
        return (starting_position_x - side_length as i32 - 1 + (x - bottom_left_value) as i32, starting_position_y-1)
    }

    // Left side?
    let top_left_value = starting_value + side_length*2+1;
    if top_left_value <= x {
        return (starting_position_x - side_length as i32 - 1, starting_position_y+side_length as i32-(x - top_left_value) as i32)
    }

    // Top side?
    let top_right_value = starting_value + side_length;
    if top_right_value <= x {
        return (starting_position_x - (x - top_right_value) as i32, starting_position_y+side_length as i32)
    }

    // Right side
    (starting_position_x, starting_position_y + (x - starting_value) as i32)
}

fn main() {
    let input = 347991;
    let (x,y) = find_position(input);
    let distance = x.abs() + y.abs();

    println!("Position ({}): ({},{}) ({})", input, x,y, distance);
}