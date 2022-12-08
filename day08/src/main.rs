#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn file_to_lines(path: &String) -> Vec<String> {
    let path = Path::new(path);

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(_) => {}
    };

    let lines : Vec<String> = s.split("\n").collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect();
    lines
}

struct Tree {
    height: usize,
    visible: bool,
}

fn lines_to_grid(lines: &Vec<String>) -> Vec<Vec<Tree>> {
    let mut grid : Vec<Vec<Tree>> = vec![];
    for l in lines {
        let tree_row : Vec<Tree> = l.chars()
                                    .map(|c| return Tree{
                                        height: c.to_string().parse::<usize>().unwrap(),
                                        visible:false })
                                    .collect();
        grid.push(tree_row);
    }

    return grid
}

fn shoot_lasers(mut grid : &mut Vec<Vec<Tree>>) {

    let size : usize = grid[0].len();

    /* mark edges */
    for i in 0..size {
        grid[0][i].visible = true;
        grid[size-1][i].visible = true;
    }
    for i in 0..size {
        grid[i][0].visible = true;
        grid[i][size-1].visible = true;
    }

    for x in 1..size-1 {
        /* shoot a beam from the left marking anything we can see */
        let mut max : usize = grid[x][0].height;
        for y in 1..size-1 {
            if grid[x][y].height > max {
                grid[x][y].visible = true;
                max = grid[x][y].height;         
            }
        }

        /* shoot a beam from the right marking anything we can see */
        max = grid[x][size-1].height;
        for y in (1..size-1).rev() {
            if grid[x][y].height > max {
                grid[x][y].visible = true;
                max = grid[x][y].height;
            }
        }
    }

    for y in 1..size-1 {
        /* shoot a beam from the top marking anything we can see */
        let mut max : usize = grid[0][y].height;
        for x in 1..size-1 {
            if grid[x][y].height > max {
                grid[x][y].visible = true;
                max = grid[x][y].height;
            }
        }

        /* shoot a beam from the bottom marking anything we can see */
        max = grid[size-1][y].height;
        for x in (1..size-1).rev() {
            if grid[x][y].height > max {
                grid[x][y].visible = true;
                max = grid[x][y].height;
            }
        }
    }
}

fn part1(path: &String) {
    println!("File: {path}");
    let lines = file_to_lines(path);
    let mut grid = lines_to_grid(&lines);
    shoot_lasers(&mut grid);

    let mut visible : usize = 0;
    for row in grid {
        for col in row {
            if col.visible {
                visible += 1;
                print!("o");
            }
            else {
                print!("X");
            }
        }
        println!("");
    }
    println!("{}", visible);
}

fn climb_trees(mut grid : &mut Vec<Vec<Tree>>) -> usize {
    let size : usize = grid[0].len();

    let mut best_score : usize = 0;

    /* climb each center tree */
    for row in 0..size {
        for col in 0..size {
            //println!("-----------");
            //println!("({},{})", row, col);
            let height = grid[row][col].height;

            let mut left_distance : usize = 0;
            let mut right_distance : usize = 0;
            let mut up_distance : usize = 0;
            let mut down_distance : usize = 0;

            for left in (0..col).rev() {
                left_distance += 1;
                if height <= grid[row][left].height {
                    break;
                }
            }

            for right in col+1..size {
                right_distance += 1;
                if height <= grid[row][right].height {
                    break;
                }
            }

            for up in (0..row).rev() {
                up_distance += 1;
                if height <= grid[up][col].height {
                    break;
                }
            }

            for down in row+1..size {
                down_distance += 1;
                if height <= grid[down][col].height {
                    break;
                }
            }

            let score = left_distance * right_distance * up_distance * down_distance;
            //println!("{} {} | {} {} {} {} => {}", row, col, left_distance, right_distance, up_distance, down_distance, score.to_string());
            if score > best_score {
                best_score = score;
            }
        }
    }
    return best_score
}

fn part2(path: &String) {

    println!("File: {path}");
    let lines = file_to_lines(path);
    let mut grid = lines_to_grid(&lines);
    let best_score = climb_trees(&mut grid);
    println!("best: {}", best_score.to_string())
}

fn main() {
    let file = std::env::args().nth(1);

    match file {
        Some(file) => part2(&file),
        None => println!("No file"),
    }
}