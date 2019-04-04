use std::io::{self, BufRead};
use std::collections::HashMap;


fn main() {
    let mut display : HashMap<&str, &str> = HashMap::new();
    display.insert(":", r#"
 
 
o
 
o
 
 "#.trim());
    // display.insert("0", );
    // display.insert("1", );
    // display.insert("2", );
    // display.insert("3", );
    // display.insert("4", );
    // display.insert("5", );    
    // display.insert("6", );    
    // display.insert("7", );    
    display.insert("8", r#"
+---+
|   |
|   |
+---+
|   |
|   |
+---+
                    "#.trim());
    // display.insert("9", );

    let stdin = io::stdin();
    
    for input in stdin.lock().lines() {
        match input {
            Err(e) => {
                panic!(e)
            },
            Ok(s) => {
                if s.trim() == "end" {
                    println!("{}", display[":"]);
                    return;
                }


            }
        }
    }
}
