use clap::Parser;

#[derive(Parser,Default,Debug)]
#[clap(author="Guillermo Oyarzun", version, about)]
///A sparse matrix-vector multiplication in CSR format
struct Arguments {
    #[clap(short, long)]
    #[clap(forbid_empty_values = true)]
    /// Matrix file in CSR format
    matrix_file: String,
}
fn main() {
    let args = Arguments::parse();
    println!("matrix_name {}", args.matrix_file);
   
    //Reading a file

    //Create a datastructure/class 

    //create the spmv kernel
    
    //measure times 

}
