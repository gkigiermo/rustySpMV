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
    #[clap(short, long)]
    #[clap(forbid_empty_values = true)]
    /// Number of times the test is repeated to obtain the average time
    repetitions: usize,
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

    //The size of the rows
    pub fn get_rows_number(&mut self) -> usize {
        self.matrix_params[2]
    }

    //The size of the columns
    pub fn get_columns_number(&mut self) -> usize {
        self.matrix_params[1]
    }

    // Just for checking the read values
    pub fn print_first_n_lines(&mut self, n: usize) -> Result<(), Box<dyn std::error::Error>>{
        for i in 0..self.matrix_params.len(){
            println!(" {}", self.matrix_params[i]);
        }
        for i in 0..n {
            println!("val[{}]: {}", i,  self.vals[i] );
            println!("col[{}]: {}", i, self.cols[i] );
            println!("row_ptr[{}]:{}", i, self.row_ptr[i] );
        }
        //Row pointer must have an extra entry
        println!(" Read row size from params : {}, obtain from row_ptr vector {} ", self.matrix_params[2], self.row_ptr.len());

        Ok(())
    }

    //Calculates _vector_y = self * _vector_x
    pub fn calculate_spmv(&mut self, _vector_x: &Vec<f64>,  _vector_y: &mut Vec<f64> ) {
        //For each row in the matrix
        for row in 0..self.get_rows_number(){
            //Read the begin and end within the vals and cols arrays
            let row_begin = self.row_ptr[row];
            let row_end = self.row_ptr[row + 1];
            let mut sum : f64 = 0.0;
            //For each non-zero element of the row calculate the inner product
            for col in row_begin..row_end{
                sum += self.vals[col] * _vector_x[self.cols[col]];
            }
            _vector_y[row] = sum;
        }

    }
}

//A simple function to fill the vector with some numbers
pub fn fill_vector(_vector : &mut Vec<f64>) {
    for index in 0.._vector.len() {
        _vector[index] = 0.1 * (index as f64);
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
    let mut y_vector: Vec<f64> = vec![0.0 ; sp_matrix.get_rows_number()];
    
    //Fill multiplying vector with values
    fill_vector(&mut x_vector);

    //Perform the sparse matrix vector multiplication
    for _repetition in 0..args.repetitions {
        sp_matrix.calculate_spmv(&x_vector, &mut y_vector);
    }
    
    //Print the first element of each vector
    println!("x: {} y: {} ", x_vector[0], y_vector[0] );

    

    //measure times 

}
