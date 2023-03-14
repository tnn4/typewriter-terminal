#[macro_use]
// Using this allows you to have static that required code to be executed at runtime in order to be initalized
extern crate lazy_static;

use std::{thread, time};

use std::fs;

use std::path::Path;



/** File Functions **/
fn read_file(path_to_file: &str) -> String{
    let contents = fs::read_to_string(path_to_file)
        .expect("Should be able to read the file."); // -> Result<String>
    contents
}
/// Print file content
fn print_file_contents(input: &String){
    println!("File contents: {input}");
}

/// Sleep for milliseconds
pub fn mssleep(ms_to_wait: u64)
{
    let time_to_wait = time::Duration::from_millis(ms_to_wait);
    thread::sleep(time_to_wait);
}

/// Takes two &str(string slices)/string literals and concatenates both then returns combined string
pub fn cat(str1: &str, str2: &str) -> String {
    format!("{}{}", str1, str2)
}

pub mod term {
    use crate::cat;

    use std::io::{self, Write};

    use colored::Colorize;
    use colored::ColoredString;
    use colored::Color;

    
    lazy_static!
    {
        static ref PAUSE_TIME: u64 = 20;
    }

    /** Type out text**/
    // Should probably combine these functions by letting it take a trait object
    /// Print out text with typewriter effect
    pub fn type_text(s: &str) 
    {
        let ms = 50;
        let sleep_time = std::time::Duration::from_millis(ms);
        for c in s.chars() {
            print!("{c}");
            std::io::stdout().flush().expect("Flushing to succeed");
            std::thread::sleep(sleep_time);
        }
    }
    
    /// Print out text with typewriter effect in X ms
    pub fn type_text_in_ms(s: &str, ms: u64) 
    {
        
        let sleep_time = std::time::Duration::from_millis(ms);
        for c in s.chars() {
            print!("{c}");
            std::io::stdout().flush().expect("Flushing to succeed");
            std::thread::sleep(sleep_time);
        }
    }

    /// This doesn't work at all 
    #[deprecated]
    pub fn type_color(s: ColoredString)
    {
        let sleep_time = std::time::Duration::from_millis(*PAUSE_TIME);
        for c in s.chars() {
            print!("{c}");
            std::io::stdout().flush().expect("Couldn't flush the terminal buffer!");
            std::thread::sleep(sleep_time);
        }
    }

    /// This doesn't work at all 
    #[deprecated]
    fn type_text_colored_in_ms(s: ColoredString, ms: u64) 
    {
        let sleep_time = std::time::Duration::from_millis(ms);
        for c in s.chars() {
            print!("{c}");
            std::io::stdout().flush().expect("Flushing to succeed");
            std::thread::sleep(sleep_time);
        }
    }
    

    /// Get information from input string
    pub fn analyze_colored_string(s: ColoredString)
    {
        let applied_color = s.fgcolor();
    
        match applied_color{
            Some(Color::Red) => println!("The color was RED!"),
            _ => println!("It was some other color or (None)"),
        }

        println!("Number of single character substrings");
        for n in 0..s.len()-1{
            print!("{n}, ");
            print!("{}", &s[n..n+1].red());
        }
        println!(" ");
    }
    
    /// Print out text with typewriter effect with a color
    /// e.g.
    /// ```
    /// type_text_colored("red", "i'm red!!!");
    /// ```
    pub fn type_text_colored(color: &str, s: &str)
    {
        let s2 = cat(s, " ");

        match color {
            "red" => {
                #[cfg(debug_assertions)]
                println!("Found: {}", color);
                type_colored_char(&s2, get_red_char);
                
                /*
                for n in 0..s2.len()-1 {
                    // print!("{}", &s[n..n+1].red());
                    print!("{}", get_red_char(&s2, n));
                    std::io::stdout().flush().expect("Flushing to succeed");
                    std::thread::sleep(std::time::Duration::from_millis(20));
                }
                */
            },
            "blue" => {
                #[cfg(debug_assertions)]
                println!("Found: {}", color);
                for n in 0..s2.len()-1 {
                    // print!("{}", &s[n..n+1].red());
                    print!("{}", get_blue_char(&s2, n));
                    std::io::stdout().flush().expect("Flushing to succeed");
                    std::thread::sleep(std::time::Duration::from_millis(20));
                }
            },
            "green" => {
                #[cfg(debug_assertions)]
                println!("Found: {}", color);
                for n in 0..s2.len()-1 {
                    // print!("{}", &s[n..n+1].red());
                    print!("{}", get_green_char(&s2, n));
                    std::io::stdout().flush().expect("Flushing to succeed");
                    std::thread::sleep(std::time::Duration::from_millis(20));
                }
            }
            ,
            _ => print!("No match")
        }
    
        

        print!("{}", &s2[s2.len()-1..]);
        println!();
    }
    
    /// Takes a callback function
    /// e.g. type_colored_char(|s2| get_red_char);
    fn type_colored_char(string: &String, f: impl Fn(&String, usize) -> ColoredString )
    {
        let sleep_time = std::time::Duration::from_millis(20);
        for i in 0..=string.len()-1 {
            // print!("{}", &s[n..n+1].red());
            print!("{}", f(string,i));
            std::io::stdout().flush().expect("Flushing to succeed");
            std::thread::sleep(sleep_time);

        }
    }


    /* get_[colored]_substring() */
    fn get_red_char(s: &String, i: usize) -> ColoredString
    {
        s[i..i+1].red()
    }
    
    fn get_green_char(s: &String, i: usize) -> ColoredString
    {
        s[i..i+1].green()
    }
    
    fn get_blue_char(s: &String, i: usize) -> ColoredString
    {
        s[i..i+1].blue()

    }
    
    fn get_char_colored(s: &String, n: usize, color: &str ){
        match color {
            "red" =>
            {
                println!("matched {}", color);
            },
            "green" => 
            {
                println!("matched {}", color);
            },
            "blue" => 
            {
                println!("matched {}", color);
            },
            _ => println!("No matching colors"),
        }
    }
}