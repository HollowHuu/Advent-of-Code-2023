pub fn part1() {
    let input = include_str!("input/day2.txt");
    let mut total = 0;
    
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    for x in input.lines() {
        let mut total_red = 0;
        let mut total_green = 0;
        let mut total_blue = 0;

        let sections = x.split(":").collect::<Vec<&str>>();
        let game_num = sections[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let hands = sections[1].split(";").collect::<Vec<&str>>();
        for i in hands.iter() {
            // Split the hand in 3 parts, calculate
            let colors = i.split(",").collect::<Vec<&str>>();
            for j in colors.iter() {
                let color = j.split_whitespace().collect::<Vec<&str>>()[1];
                let num = j.split_whitespace().collect::<Vec<&str>>()[0].parse::<i32>().unwrap();

                if color == "red" {
                    total_red += num;
                }
                else if color == "green" {
                    total_green += num;
                }
                else if color == "blue" {
                    total_blue += num;
                }
            }
        }

        if &game_num == &1 {
            println!("{} {} {}", total_red, total_green, total_blue);
        
        }

        if total_red <= max_red && total_green <= max_green && total_blue <= max_blue {
            total += game_num;
        }

    
    }
    println!("{}", total)
}