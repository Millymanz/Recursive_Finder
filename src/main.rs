use glob::glob;
use error_chain::error_chain;
use std::io::{BufRead, BufReader};
use std::fs::File;
  

 error_chain! {
     foreign_links {
         Glob(glob::GlobError);
         Pattern(glob::PatternError);
     }
}
 

fn main() {
	println!("Welcome Recursive Finder!");

    let _outcome = finder("", ".pdf");
}


fn finder(path: &str, ext: &str) -> Result<i32>{
   
    let temp_path = format!("{}{}{}", path, "**/*", ext); 
    let final_path = &temp_path;
    let mut cnt  = 0;   

    for entry in glob(final_path)? {
        let current_entry = format!("{}", entry?.display()); 

        let count = filecounter(&current_entry);       
        let countedlines = format!("{}", count.unwrap()); 

        println!("{} lines counted :- {}", current_entry, countedlines);  

        cnt = cnt + 1;
    }   
    Ok(cnt)
}

fn filecounter(path: &str) -> Result<i32> {
    let file = BufReader::new(File::open(path).expect("Unable to open file"));
    let mut cnt  = 0;    
    for _ in file.lines() {
        cnt = cnt + 1;
    }
    Ok(cnt)
}




#[cfg(test)]
mod finder_tests {
    use super::*;

    #[test]
    fn can_find_files_of_png_ext() {

    let expected_file_count = finder("",".png");
   
        assert_eq!(
            expected_file_count,
            2,
            "Should return the chosen loaded value"
        );
    }

    #[test]
    fn can_find_files_of_pdf_ext() {

    let expected_file_count = finder("",".pdf");
        assert_eq!(
            expected_file_count.unwrap(),
            3,
            "Should return the chosen loaded value"
        );
    }
}