use crate::hashvariants::{hashsetalt, hashsetref};
use crate::meanvariant::meanquality;
use crate::variantstats::statstable;
use crate::variantstruct::Genomecapture;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn zygosityfilteranalysis(pathfile: &str, zygosity: &str) -> Option<String> {
    for i in fs::read_dir(pathfile).ok()? {
        let openfile = i.ok()?.path();
        let path_str = openfile.to_str().unwrap();
        let fileopen = File::open(path_str).expect("file not found");
        let fileread = BufReader::new(fileopen);
        let mut versionfile: Vec<_> = Vec::new();
        let mut genomeanalysisvcf: Vec<Genomecapture> = Vec::new();
        let mut filteredgenomeanalysis: Vec<Genomecapture> = Vec::new();
        let filerename = path_str.split("/").collect::<Vec<_>>()[1]
            .split(".")
            .collect::<Vec<_>>()[0]
            .to_string();
        for i in fileread.lines() {
            let line = i.expect("line not present");
            if line.starts_with("#") {
                let version = line.replace("#", "");
                versionfile.push(version);
            }
            if !line.starts_with("#") && !line.starts_with("CHR") {
                let linevec = line.split("\t").collect::<Vec<_>>();
                genomeanalysisvcf.push(Genomecapture {
                    version: versionfile[0].to_string().clone(),
                    filename: filerename.clone(),
                    chr: linevec[0].to_string(),
                    start: linevec[1].to_string(),
                    end: linevec[2].to_string(),
                    generef: linevec[3].to_string(),
                    alt: linevec[4].to_string(),
                    effect: linevec[5].to_string(),
                    gene: linevec[6].to_string(),
                    transcript: linevec[7].to_string(),
                    selectcannonical: linevec[8].to_string(),
                    tfbsid: linevec[9].to_string(),
                    tfbsname: linevec[10].to_string(),
                    exonintronnum: linevec[11].to_string(),
                    hgvsc: linevec[12].to_string(),
                    hgvsp: linevec[13].to_string(),
                    cdsdistance: linevec[14].to_string(),
                    cdslen: linevec[15].to_string(),
                    aalen: linevec[16].to_string(),
                    othertranscripts: linevec[17].to_string(),
                    exac_an: linevec[18].to_string(),
                    exac_ac: linevec[19].to_string(),
                    exac_af: linevec[20].to_string(),
                    exac_istarget: linevec[21].to_string(),
                    dnsnp: linevec[22].to_string(),
                    dnsnp_version: linevec[23].to_string(),
                    dbsnp_1tgp_ref_freq: linevec[24].to_string(),
                    dbsnp_1tgp_alt_freq: linevec[25].to_string(),
                    common_1tgp_1perc: linevec[26].to_string(),
                    esp6500siv2_ea_freq: linevec[27].to_string(),
                    esp6500siv2_aa_freq: linevec[28].to_string(),
                    esp6500siv2_all_freq: linevec[29].to_string(),
                    gnomad_af_all: linevec[30].to_string(),
                    gnomad_hom_all: linevec[31].to_string(),
                    gnomad_af_max_pop: linevec[32].to_string(),
                    cadd_score: linevec[33].to_string(),
                    dbscsnv_ab_score: linevec[34].to_string(),
                    dbscsnv_rf_score: linevec[35].to_string(),
                    papi_pred: linevec[36].to_string(),
                    papi_score: linevec[37].to_string(),
                    polyphen_2_pred: linevec[38].to_string(),
                    polyphen_2_score: linevec[39].to_string(),
                    sift_pred: linevec[40].to_string(),
                    sift_score: linevec[41].to_string(),
                    pseeac_rf_pred: linevec[42].to_string(),
                    pseeac_rf_score: linevec[43].to_string(),
                    clinvar_hotspot: linevec[44].to_string(),
                    clinvar_rcv: linevec[45].to_string(),
                    clinvar_clinical_significance: linevec[46].to_string(),
                    clinvar_rev_status: linevec[47].to_string(),
                    clinical_traits: linevec[48].to_string(),
                    clinvar_traitsclinvar_pmids: linevec[49].to_string(),
                    diseases: linevec[50].to_string(),
                    disease_ids: linevec[51].to_string(),
                    geno: linevec[52].to_string(),
                    qual: linevec[53].to_string(),
                    geno_qual: linevec[54].to_string(),
                    genofilter: linevec[55].to_string(),
                    af: linevec[56].to_string(),
                    ao: linevec[57].to_string(),
                    ro: linevec[58].to_string(),
                    co: linevec[59].to_string(),
                });
            }
        }
        for i in genomeanalysisvcf.iter() {
            if i.geno == zygosity {
                filteredgenomeanalysis.push(Genomecapture {
                    version: i.version.clone(),
                    filename: i.filename.clone(),
                    chr: i.chr.to_string(),
                    start: i.start.to_string(),
                    end: i.end.to_string(),
                    generef: i.generef.to_string(),
                    alt: i.alt.to_string(),
                    effect: i.effect.to_string(),
                    gene: i.gene.to_string(),
                    transcript: i.transcript.to_string(),
                    selectcannonical: i.selectcannonical.to_string(),
                    tfbsid: i.tfbsid.to_string(),
                    tfbsname: i.tfbsname.to_string(),
                    exonintronnum: i.exonintronnum.to_string(),
                    hgvsc: i.hgvsc.to_string(),
                    hgvsp: i.hgvsp.to_string(),
                    cdsdistance: i.cdsdistance.to_string(),
                    cdslen: i.cdslen.to_string(),
                    aalen: i.aalen.to_string(),
                    othertranscripts: i.othertranscripts.to_string(),
                    exac_an: i.exac_an.to_string(),
                    exac_ac: i.exac_ac.to_string(),
                    exac_af: i.exac_af.to_string(),
                    exac_istarget: i.exac_istarget.to_string(),
                    dnsnp: i.dnsnp.to_string(),
                    dnsnp_version: i.dnsnp_version.to_string(),
                    dbsnp_1tgp_ref_freq: i.dbsnp_1tgp_ref_freq.to_string(),
                    dbsnp_1tgp_alt_freq: i.dbsnp_1tgp_alt_freq.to_string(),
                    common_1tgp_1perc: i.common_1tgp_1perc.to_string(),
                    esp6500siv2_ea_freq: i.esp6500siv2_ea_freq.to_string(),
                    esp6500siv2_aa_freq: i.esp6500siv2_aa_freq.to_string(),
                    esp6500siv2_all_freq: i.esp6500siv2_all_freq.to_string(),
                    gnomad_af_all: i.gnomad_af_all.to_string(),
                    gnomad_hom_all: i.gnomad_hom_all.to_string(),
                    gnomad_af_max_pop: i.gnomad_af_max_pop.to_string(),
                    cadd_score: i.cadd_score.to_string(),
                    dbscsnv_ab_score: i.dbscsnv_ab_score.to_string(),
                    dbscsnv_rf_score: i.dbscsnv_rf_score.to_string(),
                    papi_pred: i.papi_pred.to_string(),
                    papi_score: i.papi_score.to_string(),
                    polyphen_2_pred: i.polyphen_2_pred.to_string(),
                    polyphen_2_score: i.polyphen_2_score.to_string(),
                    sift_pred: i.sift_pred.to_string(),
                    sift_score: i.sift_score.to_string(),
                    pseeac_rf_pred: i.pseeac_rf_pred.to_string(),
                    pseeac_rf_score: i.pseeac_rf_score.to_string(),
                    clinvar_hotspot: i.clinvar_hotspot.to_string(),
                    clinvar_rcv: i.clinvar_rcv.to_string(),
                    clinvar_clinical_significance: i.clinvar_clinical_significance.to_string(),
                    clinvar_rev_status: i.clinvar_rev_status.to_string(),
                    clinical_traits: i.clinical_traits.to_string(),
                    clinvar_traitsclinvar_pmids: i.clinvar_traitsclinvar_pmids.to_string(),
                    diseases: i.diseases.to_string(),
                    disease_ids: i.disease_ids.to_string(),
                    geno: i.geno.to_string(),
                    qual: i.qual.to_string(),
                    geno_qual: i.geno_qual.to_string(),
                    genofilter: i.genofilter.to_string(),
                    af: i.af.to_string(),
                    ao: i.ao.to_string(),
                    ro: i.ro.to_string(),
                    co: i.co.to_string(),
                });
                let hashvariant_ref_before_filter: HashSet<String> =
                    hashsetref(genomeanalysisvcf.clone()).unwrap();
                let hashvariant_alt_before_filter: HashSet<String> =
                    hashsetalt(filteredgenomeanalysis.clone()).unwrap();
                let _variantstats_before_filter =
                    statstable(genomeanalysisvcf.clone(), &i.filename.clone()).unwrap();
                let _variantstats_after_filter =
                    statstable(filteredgenomeanalysis.clone(), &i.filename.clone()).unwrap();

                let mut variant_ref = File::create(format!(
                    "{}.{}",
                    i.filename.clone(),
                    "ref-unique-variants.txt"
                ))
                .expect("file not present");
                let mut variant_alt = File::create(format!(
                    "{}.{}",
                    i.filename.clone(),
                    "alt-unique-variants.txt"
                ))
                .expect("file not present");
                for i in hashvariant_ref_before_filter.iter() {
                    writeln!(variant_ref, "{}", i).expect("file not found");
                }
                for i in hashvariant_alt_before_filter.iter() {
                    writeln!(variant_alt, "{}", i).expect("file not found");
                }

                let meanvalue_before_filter = meanquality(genomeanalysisvcf.clone()).unwrap();
                let meanvalue_after_filter = meanquality(filteredgenomeanalysis.clone()).unwrap();

                let writefilename = format!("{}.{}", i.filename.clone(), "filtered");
                let mut filewrite = File::create(writefilename).expect("file not present");
                writeln!(
                    filewrite,
                    "{}\t{}",
                    "The mean quality for the file before filtering is:",
                    meanvalue_before_filter.to_string()
                )
                .expect("file not present");
                writeln!(
                    filewrite,
                    "{}\t{}",
                    "The mean quality for the file before filtering is:",
                    meanvalue_after_filter.to_string()
                )
                .expect("file not present");
                writeln!(filewrite, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                      "version",
                      "filename",
                      "CHR",
            "START",
            "END",
            "REF",
            "ALT",
            "EFFECT",
            "GENE",
            "TRANSCRIPT_ID",
            "SELECT_CANONICAL",
            "TFBS_Id",
            "TFBS_name",
            "EXON_INTRON_NUM",
            "HGVS_C",
            "HGVS_P",
            "CDS_DISTANCE",
            "CDS_LEN",
            "AA_LEN",
            "OTHER_TRANSCRIPTS",
            "ExAC_AN",
            "ExAC_AC",
            "ExAC_AF",
            "ExAC_isTarget",
            "DBSNP",
            "DBSNP_VERSION",
            "DBSNP_1TGP_REF_freq",
            "DBSNP_1TGP_ALT_freq",
            "COMMON_1TGP_1perc",
            "ESP6500SIv2_EA_freq",
            "ESP6500SIv2_AA_freq",
            "ESP6500SIv2_All_freq",
            "gnomAD_AF_ALL",
            "gnomAD_Hom_ALL",
            "gnomAD_AF_MAX_POP",
            "CADD_score",
            "dbscSNV_AB_score",
            "dbscSNV_RF_score",
            "PaPI_pred",
            "PaPI_score",
            "PolyPhen-2_pred",
            "PolyPhen-2_score",
            "SIFT_pred",
            "SIFT_score",
            "PseeAC-RF_pred",
            "PseeAC-RF_score",
            "ClinVar_hotSpot",
            "ClinVar_RCV",
            "ClinVar_clinical_significance",
            "ClinVar_rev_status",
            "ClinVar_traits",
            "ClinVar_PMIDS",
            "Diseases",
            "Disease_IDs",
            "GENO",
            "QUAL",
            "GENO_QUAL",
            "FILTER",
            "AF",
            "AO",
            "RO",
            "COV").expect("file not present");
                for i in filteredgenomeanalysis.iter() {
                    writeln!(filewrite, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", i.version, i.filename, i.chr, i.start, i.end, i.generef, i.alt, i.effect, i.gene, i.transcript, i.selectcannonical, i.tfbsid, i.tfbsname, i.exonintronnum, i.hgvsc, i.hgvsp, i.cdsdistance, i.cdslen, i.aalen, i.othertranscripts, i.exac_an, i.exac_ac, i.exac_af, i.exac_istarget, i.dnsnp, i.dnsnp_version, i.dbsnp_1tgp_ref_freq, i.dbsnp_1tgp_alt_freq, i.common_1tgp_1perc, i.esp6500siv2_ea_freq, i.esp6500siv2_aa_freq, i.esp6500siv2_all_freq, i.gnomad_af_all, i.gnomad_hom_all, i.gnomad_af_max_pop, i.cadd_score,i.dbscsnv_ab_score, i.dbscsnv_rf_score, i.papi_pred,i.papi_score,i.polyphen_2_pred,i.polyphen_2_score, i.sift_pred, i.sift_score,i.pseeac_rf_pred,i.pseeac_rf_score,i.clinvar_hotspot,i.clinvar_rcv, i.clinvar_clinical_significance, i.clinvar_rev_status, i.clinical_traits, i.clinvar_traitsclinvar_pmids, i.diseases, i.disease_ids, i.geno, i.qual, i.geno_qual, i.genofilter, i.af, i.ao, i.ro,i.co).expect("file not present");
                }
            }
        }
    }
    Some("The folder has been analyzed".to_string())
}
