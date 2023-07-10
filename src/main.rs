use std::io::{self, BufReader};
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;


#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long)]
    fastq: String,
    #[arg(short, long, default_value_t = String::from("quality_distro.csv"))]
    output: String
}

#[derive(Debug)]
struct ScoreCalculator {
    fastq_file: String,
    output_file: String,
    score_memo: [u64; 51]
}

impl ScoreCalculator {
    fn update_quality_score(&mut self, qual_string: &String) -> () {
        //! Takes in a sequence and a "memoization" (memo) structure.
        //! Loops through the sequence, calculates the score,
        //! and updates the memo
        //! Inputs
        //!     qual_string - a quality score string; assumes Phred33
        //!     score_memo - a vector of quality scores that is perpetually updated
        //! Outputs
        //!     a vector of the updated memo
        //let mut quality_score: usize = 0;
        for base in qual_string.chars() {
            let quality_score = (base as u8 - 33) as usize;
            if quality_score > 50 {
                println!("There is a weird quality score of {}", quality_score);
            } else {
                self.score_memo[quality_score] += 1;
            }
        }
    }

    fn parse_fastq(&mut self) -> io::Result<()> {
        let file = File::open(self.fastq_file.clone())?;
        let file = BufReader::new(file);
        let mut counter: u64 = 0;
        for line in file.lines() {
            counter += 1;
            let line = match line {
                Ok(file) => file,
                Err(error) => panic!("There was a problem opening the file: {:?}", error)
            };
            match counter % 4 {
                1 => (),
                2 => (),
                3 => (),
                0 => self.update_quality_score(&line),
                _ => println!("There is bad modulo logic")
            }
        }

        Ok (())
    }

    fn write_quality_scores(&self) -> io::Result<()> {
        let mut file = File::create(self.output_file.clone())?;
        write!(file, "quality_score,counts\n");
        for (score, count) in self.score_memo.iter().enumerate() {
            write!(file, "{score},{count}\n");
        }

        Ok (())
    }
}


fn main() {
    let cli = Cli::parse(); // get arguments
    let mut qual_score = ScoreCalculator{
        fastq_file: cli.fastq,
        output_file: cli.output,
        score_memo: [0; 51]
    };

    //let parser = qual_score.parse_fastq();
    qual_score.parse_fastq();
    qual_score.write_quality_scores();
}