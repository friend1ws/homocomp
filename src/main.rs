use std::env; 
use std::error::Error;
use bio::io::fasta;

fn main() -> Result<(),  Box<dyn Error>> {

    let input_file_path = env::args_os().nth(1).expect("No argument");
    let reader = fasta::Reader::from_file(input_file_path)?;
 
    for result in reader.records() {
        let record = result?;
        let seq_vec = record.seq(); 

        // eprintln!("Processing {}", record.id());

        let mut comp_seq: Vec<u8> = Vec::new();
        let mut prev_chr: u8 = 0;
        for temp_chr_ptr in seq_vec.iter() {
            if *temp_chr_ptr != prev_chr && prev_chr != 0 {
                comp_seq.push(prev_chr);
            } 
            prev_chr = *temp_chr_ptr;
 
        }

        comp_seq.push(prev_chr);
        let comp_seq_str: String = String::from_utf8(comp_seq).unwrap();
        println!(">{}\n{}", record.id(), comp_seq_str);

    }

    Ok(())
}
