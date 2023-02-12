// Copyright (c) 2023 The MobileCoin Foundation

#![deny(missing_docs)]

//! Intel SGX SIGSTRUCT Dump Utility
//!
//! This utility will read a SIGSTRUCT (css) file from standard input or the
//! command-line, print it's contents (optionally as debug byte arrays) to
//! standard output or an output file.

use clap::Parser;
use mc_sgx_css::Signature;
use std::{
    fs,
    io::{self, Read, Write as IoWrite},
    mem,
    path::PathBuf,
};

#[derive(Debug, Parser)]
struct Config {
    /// The SIGSTRUCT file to read, or stdin
    #[clap(value_parser)]
    pub input: Option<PathBuf>,
    /// The output location, or stdout
    #[clap(value_parser)]
    pub output: Option<PathBuf>,
}

fn main() {
    let config = Config::parse();

    let input = match config.input {
        Some(input) => fs::read(input).expect("Could not read input file"),
        None => {
            // sigstruct structures should be 1208 bytes
            let mut bytes = vec![0u8; mem::size_of::<Signature>()];
            io::stdin()
                .read_exact(&mut bytes)
                .expect("Could not read SIGSTRUCT from standard input");
            bytes
        }
    };

    let sigstruct = Signature::try_from(input.as_slice()).expect("Could not parse input");
    let output = format!("{sigstruct:#}");
    match config.output {
        Some(outfile) => {
            fs::write(outfile, output.as_bytes()).expect("Could not write output file");
        }
        None => io::stdout()
            .write_all(output.as_bytes())
            .expect("Could not write output to standard out"),
    }
}
