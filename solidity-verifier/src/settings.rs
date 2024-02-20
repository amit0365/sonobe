use clap::{Parser, ValueEnum};
use std::{env, fmt::Display, path::PathBuf};

fn get_default_out_path() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push("verifier.sol");
    path
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub(crate) enum Protocol {
    NovaCyclefold,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// We want this to:
// Generate a verifier for a specified protocol given the vkey of
// KZG and G16 passed by path and serialized with Arkworks.

#[derive(Debug, Parser)]
#[command(author = "0XPARC & PSE", version, about, long_about = None)]
#[command(propagate_version = true)]
/// A tool to create Solidity Contracts which act as verifiers for the major Folding Schemes implemented
/// within the `folding-schemes` repo.
pub(crate) struct Cli {
    #[command(flatten)]
    pub verbosity: clap_verbosity_flag::Verbosity,

    /// Selects the protocol for which we want to generate the Decider circuit Solidity Verifier.
    #[arg(short, long, value_enum, rename_all = "lower")]
    pub protocol: Protocol,

    #[arg(short, long, default_value=get_default_out_path().into_os_string())]
    /// Sets the output path for all the artifacts generated by the command.
    pub out: PathBuf,

    #[arg(short, long)]
    /// Sets the input path for the VerifierKey of the Groth 16 circuit.
    pub g16_vkey: PathBuf,

    #[arg(short, long)]
    /// Sets the input path for the VerifierKey of the KZG setup.
    pub kzg_vkey: Option<PathBuf>,
}