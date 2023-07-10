# fastq_stats
calculate fastq statistics

# Requirements
rust
clap = 4.3.0 (module for rust)

# Intro
This code will take in a fastq file and output a csv list of quality score and their counts. This
allows users to easily calculate q30 statistics (or really, q anything). The maximum quality score
that will be calculated is 50. This code was written in rust so as to more rapidly process the fastq
file when compared to python scripts and also to practice rust coding.

In the future, this code might do more including:
1. compressed fastq file handling
2. plotting the statistics
3. calculating GC content of reads
4. calculate number of kmers

# How to use
```
read_distro_calculator -f uncompressed_file.fq -o output_file.csv
```

# How to build
If you're new to rust and want to know how to build this, simply install rust, download these files and run
```
cargo build
```
in the directory containing these files.
