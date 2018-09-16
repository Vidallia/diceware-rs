

pub struct OutputDisplay;

impl OutputDisplay {
    pub fn print(output_data: &Vec<String>) {
        let longest = OutputDisplay::longest_phrase(output_data);
        let top_bottom = format!("+{}+", "-".repeat(longest + 3));
        for s in output_data {
            let space_sz = (longest - s.len()) + 1;
            println!("{}",top_bottom);
            println!("|  {}{}|", s, " ".repeat(space_sz));
        }
        println!("{}",top_bottom);
    }

    fn longest_phrase(output_data: &Vec<String>) -> usize {
        output_data.iter()
                    .map(|s| s.len() )
                    .fold(0, |acc, n| match n > acc {
                        true => n,
                        false => acc,
                    }) 
    }

}
