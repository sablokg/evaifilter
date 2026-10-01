use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "evaifilter",
    version = "1.0",
    about = "asynchronous evai vcf filter for human genomics.
       ************************************************
       Gaurav Sablok,
       gsablok@proton.me
      ************************************************ "
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// vcf filter according to quality, coverage and zygosity
    DefaultVCFFilter {
        /// path to the vcf file
        vcffile: String,
        /// threads for the analysis
        thread: String,
    },
    /// vcf filter according to quality
    QualityVCFFilter {
        /// path to the vcf file
        vcffile: String,
        /// quality of the predicted variant
        quality: String,
        /// threads for the analysis
        thread: String,
    },
    /// vcf filter according to coverage
    CoverageVCFFilter {
        /// path to the vcf file
        vcffile: String,
        /// coverage of the predicted variant
        coverage: String,
        /// thread for the analysis
        thread: String,
    },
    /// vcf filter according to zygosity
    ZygosityVCFFilter {
        /// path to the vcf file
        vcffile: String,
        /// zygosity of the predicted variant
        zygosity: String,
        /// thread for the analysis
        thread: String,
    },
    /// vcf filter default quality40, coverage10 and zygosity multi
    VariantDefaultVCFFilter {
        /// path to the vcf file
        vcffile: String,
        /// variant to be filtered
        variant: String,
        /// thread for the analysis
        thread: String,
    },
    /// vcf filter according to quality and variant
    VariantQualityVCFFilter {
        /// path to the vcf file
        vcffile: String,
        /// quality of the predicted variant
        quality: String,
        /// variant to be filtered
        variant: String,
        /// thread for the analysis
        thread: String,
    },
    /// vcf filter according to coverage and variant
    VariantCoverageVCFFilter {
        /// path to the vcf file
        vcffile: String,
        /// coverage of the predicted variant
        coverage: String,
        /// variant to be filtered
        variant: String,
        /// thread for the analysis
        thread: String,
    },
    /// vcf filter according to zygosity and variant
    VariantZygosityVCFFilter {
        /// path to the vcf file
        vcffile: String,
        /// zygosity of the predicted variant
        zygosity: String,
        /// variant to be filtered
        variant: String,
        /// thread for the analysis
        thread: String,
    },
}
