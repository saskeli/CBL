use bincode::serialize_into;
use cbl::cbl::*;
use cbl::reads::*;
use clap::Parser;
use std::fs::File;
use std::io::BufWriter;

// Loads runtime-provided constants for which declarations
// will be generated at `$OUT_DIR/constants.rs`.
pub mod constants {
    include!(concat!(env!("OUT_DIR"), "/constants.rs"));
}

use constants::{K, M, NT, PREFIX_BITS};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file (.fasta, .fa)
    input: String,
    /// Output file (defaults to <input>.cbl)
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input_filename = args.input.as_str();
    let output_filename = if let Some(filename) = args.output {
        filename
    } else {
        input_filename.to_owned() + ".cbl"
    };

    let mut cbl = CBL::<K, NT, PREFIX_BITS, M>::new();
    let reads = Fasta::from_file(input_filename);

    reads.process_rec(|rec| {
        cbl.insert_seq(rec.seq());
    });

    let output = File::create(output_filename).expect("Failed to open output file");
    let mut writer = BufWriter::new(output);
    serialize_into(&mut writer, &cbl).unwrap();
}
