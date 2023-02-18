use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Parser,Default,Debug)]
#[clap(author="Guillermo Oyarzun", version, about)]
/// A sparse matrix-vector multiplication in CSR format
struct Arguments {
    #[clap(short, long)]
    #[clap(forbid_empty_values = true)]
    /// Matrix file in CSR format
    matrix_file: String,
}

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    //Matrix params and values::: missing convert this into an optimal data structure
    let mut matrix_params: Vec<i32> = Vec::new();
    let mut matrix_vals: Vec<f64> = Vec::new();
    let mut matrix_cols: Vec<i32> = Vec::new();
    let mut matrix_row_ptrs: Vec<i32> = Vec::new();
    let mut index : i32 = 0;

    for line in reader.lines() {
        for word in line?.split(' '){
            if index == 0 {
                matrix_params.push(word.parse::<i32>().unwrap());
            } else if index == 1 {
                matrix_vals.push(word.parse::<f64>().unwrap());
            } else if index == 2 {
                matrix_cols.push(word.parse::<i32>().unwrap());
            } else if index == 3 {
                matrix_row_ptrs.push(word.parse::<i32>().unwrap());
            }
        }
        index += 1;
    }

    // Just a small check
    for i in 0..5 {
        println!("param: {}", matrix_params[i] );
        println!("val: {}", matrix_vals[i] );
        println!("col: {}", matrix_cols[i] );
        println!("row_ptr:{}", matrix_row_ptrs[i] );
    }

    Ok(())
}

fn main() {
    let args = Arguments::parse();
    let result = read_file_line_by_line(&args.matrix_file);
    
    match result {
        Ok(()) => println!("Matrix read correctly"),
        Err(e) => println!("Error: {}", e ),
    }
    //println!("matrix_name {}", args.matrix_file);
   
    //Reading a file

    //Create a datastructure/class 

    //create the spmv kernel
    
    //measure times 

}
