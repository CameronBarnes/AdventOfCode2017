#[derive(Clone, Copy)]
enum Dir {
    Up,
    Left,
    Down,
    Right
}

fn step_to_pos(num: i32) -> (i32, i32) {

    if num == 1 {
        return (0, 0);
    }

    let complete_block_size_res = f32::sqrt(num as f32);
    let mut block_size = complete_block_size_res as i32;
    let half_block = (block_size as f32 / 2.0).ceil() as i32;
    if block_size % 2 == 0 {
        block_size -= 1;
    }

    if block_size as f32 == complete_block_size_res {
        return (half_block - 1, half_block - 1)
    }

    let mut count = block_size.pow(2) + 1;
    let mut x = half_block;
    let mut y = half_block - 1; // Current position at this point is the first num on the new layer
    let mut dir = Dir::Up;

    while num > count {
        match dir {
            Dir::Up => {
                y -= 1;
                if y <= -half_block {
                    y = -half_block;
                    dir = Dir::Left;
                }
            },
            Dir::Left => {
                x -= 1;
                if x <= -half_block {
                    x = -half_block;
                    dir = Dir::Down;
                }
            },
            Dir::Down => {
                y += 1;
                if y >= half_block {
                    y = half_block;
                    dir = Dir::Right;
                }
            },
            Dir::Right => {
                x += 1;
                if x >= half_block {
                    panic!("filled out the next shell");
                }
            },
        }
        count += 1;
    }

    (x, y)

}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    
    let input = input.trim().parse::<i32>().expect("Input is always an int");
    let (x, y) = step_to_pos(input);
    (x.abs() + y.abs()).to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!("0", process("1"));
        assert_eq!("3", process("12"));
        assert_eq!("2", process("23"));
        assert_eq!("31", process("1024"));
    }
}

