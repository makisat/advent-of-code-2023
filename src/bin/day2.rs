use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("error reading input");

    part1(&input);
}

fn part1(input: &str) {
    let each_line = input.lines();

    let mut total = 0;

    for line in each_line {

        if line.is_empty() {
            break
        };

        let mut line_result: HashMap<&str, &str> = HashMap::new();

        let id_content: Vec<&str> = line.split(": ").collect();
        let game_id = id_content[0].split(" ").collect::<Vec<&str>>()[1];


        let each_grab: Vec<&str> = id_content[1].split("; ").collect();
        for (_, val) in each_grab.iter().enumerate() {
            let each_color: Vec<&str> = val.split(", ").collect();

            for colors in each_color.iter() {
                let num_color: Vec<&str> = colors.split(" ").collect();

                match line_result.get(num_color[1]) {
                    None => {
                        line_result.insert(num_color[1], num_color[0]);
                    },
                    Some(x) => {
                        let x = x.parse::<u32>().unwrap();
                        if x < num_color[0].parse::<u32>().unwrap() {
                            line_result.insert(num_color[1], num_color[0]);
                        }
                    },
                };
            }
        }

        let mut is_possible = false;
        for (key, val) in line_result.iter() {
            let val_i = val.parse::<u32>().unwrap();
            match key {
                &"red" => {
                    if val_i > 12 {
                        is_possible = false;
                        break;
                    };
                },
                &"green" => {
                    if val_i > 13 {
                        is_possible = false;
                        break;
                    };
                },
                &"blue" => {
                    if val_i > 14 {
                        is_possible = false;
                        break;
                    };
                },
                _ => {
                    is_possible = false;
                    break;
                },
            };
            is_possible = true;
        }
        if is_possible {
            total += game_id.parse::<u32>().unwrap();
        }
    }

    println!("total: {}", total);
}
