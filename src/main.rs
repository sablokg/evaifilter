mod args;
mod coveragefilter;
mod coveragefilter_with_variant;
mod defaultfilter;
mod defaultfilter_with_variant;
mod hashvariants;
mod meanvariant;
mod qualityfilter;
mod qualityfilter_with_variant;
mod variantstats;
mod variantstruct;
mod zygosityfilter;
mod zygosityfilter_with_variant;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::coveragefilter::coveragefilteranalysis;
use crate::coveragefilter_with_variant::variantcoveragefilteranalysis;
use crate::defaultfilter::defaultfilteranalysis;
use crate::defaultfilter_with_variant::variantdefaultfilteranalysis;
use crate::qualityfilter::qualityfilteranalysis;
use crate::qualityfilter_with_variant::variantqualityfilteranalysis;
use crate::zygosityfilter::zygosityfilteranalysis;
use crate::zygosityfilter_with_variant::variantzygosityfilteranalysis;
use clap::Parser;
use figlet_rs::FIGfont;

/*
Gaurav Sablok,
gsablok@proton.me
*/

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("evaifilter");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::DefaultVCFFilter { vcffile, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = defaultfilteranalysis(vcffile).unwrap();
                println!("The folder has been filtered:{:?}", command);
            });
        }
        Commands::CoverageVCFFilter {
            vcffile,
            coverage,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = coveragefilteranalysis(vcffile, &*coverage).unwrap();
                println!("The folder has been filtered:{:?}", command);
            });
        }
        Commands::QualityVCFFilter {
            vcffile,
            quality,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = qualityfilteranalysis(vcffile, &*quality).unwrap();
                println!("The folder has been filtered:{:?}", command);
            });
        }
        Commands::ZygosityVCFFilter {
            vcffile,
            zygosity,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = zygosityfilteranalysis(vcffile, zygosity).unwrap();
                println!("The folder has been filtered:{:?}", command);
            });
        }
        Commands::VariantCoverageVCFFilter {
            vcffile,
            coverage,
            variant,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = variantcoveragefilteranalysis(vcffile, coverage, variant).unwrap();
                println!(
                    "The command has been finished and the file has been written:{:?}",
                    command
                );
            });
        }
        Commands::VariantDefaultVCFFilter {
            vcffile,
            variant,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = variantdefaultfilteranalysis(vcffile, variant).unwrap();
                println!("The command has been finished:{:?}", command);
            });
        }
        Commands::VariantZygosityVCFFilter {
            vcffile,
            zygosity,
            variant,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = variantzygosityfilteranalysis(vcffile, zygosity, variant).unwrap();
                println!("The command has finished:{:?}", command);
            });
        }
        Commands::VariantQualityVCFFilter {
            vcffile,
            quality,
            variant,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = variantqualityfilteranalysis(vcffile, quality, variant).unwrap();
                println!("The command has been finished:{:?}", command);
            });
        }
    }
}
