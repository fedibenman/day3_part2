use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Replace "your_file.txt" with the path to your text file
    let file = File::open("file.txt")?;
    let reader = io::BufReader::new(file);
    let mut digit_start = 0;
    let mut number_indices: Vec<(i32, usize, usize)> = Vec::new();
    // Variable to store the sum of valid series of digits
    let mut sum: i32 = 0;

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // Iterate over lines in the file
    for (j, line) in lines.iter().enumerate() {
        // Iterate over characters in the line
        let mut chars: Vec<char> = line.chars().collect();
        let mut current_series = String::new();
        chars.push('.'); // this solves the edge case where the number is at the end of the line
        for (i, &c) in chars.iter().enumerate() {
            // Check if the character is a digit
            if c.is_digit(10) {
                if current_series.is_empty() {
                    digit_start = i;
                }
                // Add the digit to the current series
                current_series.push(c);
            } else if !current_series.is_empty() {
                // Check for adjacent characters that are not digits or dots
                let prev_char = if digit_start==0 { *chars.get(digit_start).unwrap_or(&' ') }else{ *chars.get(digit_start - 1).unwrap_or(&' ')};
                let next_char = *chars.get(digit_start+current_series.len()).unwrap_or(&' ');
                // Collect lines above and below
                let empty_string = "".to_string();
let up_line = if j > 0 {    
    lines.get(j - 1).unwrap_or(&empty_string)
} else {
    &empty_string
};

let down_line = if j < lines.len() - 1 {
    lines.get(j + 1).unwrap_or(&empty_string)
} else {
    &empty_string
};
                // Collect characters based on the current digit series indices
                let previous_up_chars = if digit_start > 0 { up_line.chars().skip(digit_start - 1).take(current_series.len() + 2).collect::<String>()} else{up_line.chars().skip(digit_start).take(current_series.len() + 1).collect::<String>()} ;
                let previous_down_chars = if digit_start > 0 {down_line.chars().skip(digit_start - 1).take(current_series.len() + 2).collect::<String>()} else{down_line.chars().skip(digit_start).take(current_series.len() + 1).collect::<String>()} ;
let index_in_up = previous_up_chars.chars().position(|c:char| c== '*') ; 
let index_in_down = previous_down_chars.chars().position(|c:char| c== '*') ;
            if previous_up_chars.contains(|c: char| c == '*') {
                
                    if let Ok(number) = current_series.parse::<i32>() {
                        number_indices.push((number, j-1 ,if digit_start>0 {digit_start-1+ index_in_up.unwrap_or_default()} else {digit_start+index_in_up.unwrap_or_default()} ));
                  println!("{:?}",(number, j-1 ,if digit_start>0 {digit_start-1+ index_in_up.unwrap_or_default()} else {digit_start+index_in_up.unwrap_or_default()} ))
                    }
            }
            
            if prev_char == '*'  {
                
                    if let Ok(number) = current_series.parse::<i32>() {
                        number_indices.push((number, j , digit_start-1 ));
                    println!("{:?}" ,(number, j , digit_start-1 )) ;
                    }
                
            }
            if next_char == '*'
                {
                        if let Ok(number) = current_series.parse::<i32>() {
                            number_indices.push((number, j ,if digit_start>0 {digit_start+ current_series.len()} else {digit_start+1+current_series.len()}));
                            println!("{:?}" , (number, j ,if digit_start>0 {digit_start+ current_series.len()} else {digit_start+1+current_series.len()}));
                        }
                    
                }

                if previous_down_chars.contains(|c: char| c == '*') {
                        if let Ok(number) = current_series.parse::<i32>() {
                            number_indices.push((number, j+1 ,if digit_start>0 {digit_start-1+ index_in_down.unwrap_or_default()} else {digit_start+index_in_down.unwrap_or_default()} ));
                    println!("{:?}" , (number, j+1 ,if digit_start>0 {digit_start-1+ index_in_down.unwrap_or_default()} else {digit_start+index_in_down.unwrap_or_default()} )   ) ;
                    }
                }

                // Reset the current series
                current_series.clear();
            }
        }
    }


    for i in 0..number_indices.len() {
        for j in i + 1..number_indices.len() {
            if number_indices[i].1 == number_indices[j].1 && number_indices[i].2 ==number_indices[j].2 {
          //      println!("the first index{} the second index {} the first number {} the second number{} " ,number_indices[i].2 , number_indices[j].1 ,number_indices[j].0 ,number_indices[i].0 );
                let product = number_indices[i].0 * number_indices[j].0;
                sum+= product;
            }
        }
    }

    // Print the sum of valid series of digits
    println!("Sum of valid series of digits: {}", sum);

    Ok(())
}
