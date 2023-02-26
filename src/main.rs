use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Parser,Default,Debug)]
#[clap(author="Guillermo Oyarzun", version, about)]
/// A sparse matrix-vector multiplication in CSR format.
/// Calculates: y = A * x
/// , where:
/// A is a sparse matrix in CSR format,
/// x is a dense vector with as many components as columns in the sparse matrix,
/// y is a dense vector with as many components as rows in the sparse matrix.

struct Arguments {
    #[clap(short, long)]
    #[clap(forbid_empty_values = true)]
    /// Matrix file in CSR format
    matrix_file: String,
}

struct SparseMatrix{
    matrix_params : Vec<usize>,
    vals : Vec<f64>,
    cols : Vec<usize>,
    row_ptr: Vec<usize>,
}

impl SparseMatrix{
    //Creating an empty sparse matrix
    pub fn new()-> Self{
        let matrix_params:  Vec<usize> = Vec::new();
        let vals:  Vec<f64> = Vec::new();
        let cols:  Vec<usize> = Vec::new();
        let row_ptr:  Vec<usize> = Vec::new();

        Self{
            matrix_params,
            vals,
            cols,
            row_ptr,
        }
    }
    // Reading the matrix in the current format
    pub fn read_matrix_from_file(&mut self, filepath: &str)-> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
        
        let mut index : i32 = 0;
        for line in reader.lines() {
            for word in line?.split(' '){
                if index == 0 {
                    self.matrix_params.push(word.parse::<usize>().unwrap().try_into().unwrap());
                } else if index == 1 {
                    self.vals.push(word.parse::<f64>().unwrap());
                } else if index == 2 {
                    self.cols.push(word.parse::<usize>().unwrap().try_into().unwrap());
                } else if index == 3 {
                    self.row_ptr.push(word.parse::<usize>().unwrap().try_into().unwrap());
                }
            }
            index += 1;
        }

        Ok(())
    }

    pub fn get_rows_number(&mut self) -> usize {
        self.matrix_params[2]
    }

    pub fn get_columns_number(&mut self) -> usize {
        self.matrix_params[1]
    }

    pub fn print_first_n_lines(&mut self, n: usize) -> Result<(), Box<dyn std::error::Error>>{
        for i in 0..self.matrix_params.len(){
            println!(" {}", self.matrix_params[i]);
        }
        for i in 0..n {
            println!("val[{}]: {}", i,  self.vals[i] );
            println!("col[{}]: {}", i, self.cols[i] );
            println!("row_ptr[{}]:{}", i, self.row_ptr[i] );
        }

        Ok(())
    }
}

pub fn fill_vector(_vector : &mut Vec<f64>) {
    for index in 0.._vector.len() {
        _vector[index] = 0.1;
    }
}

fn main() {
    //Read the arguments given in the console
    let args = Arguments::parse();

    //Create an empty matrix
    let mut sp_matrix = SparseMatrix::new();
    
    //Fill the structure with the matrix information from the file
    match &sp_matrix.read_matrix_from_file(&args.matrix_file){
        Ok(()) => println!("Matrix from struct read correctly"),
        Err(e) => println!("Error: {}", e ),    
    }
    
    //Just a check to print the first n elements in the vectors
    match sp_matrix.print_first_n_lines(1){
        Ok(()) => {},
        Err(e) => println!("Error: {}", e ),  
    }
    
    //Create multiplying vector
    let mut x_vector: Vec<f64> = vec![0.0 ; sp_matrix.get_columns_number()];
    
    //Create resulting vector
    let y_vector: Vec<f64> = vec![0.0 ; sp_matrix.get_rows_number()];
    
    fill_vector(&mut x_vector);

    println!("x: {} y: {} ", x_vector[0], y_vector[0] );

    //create the spmv kernel
    

    //measure times 

}
