#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - A/D Control Register"]
    pub adcsr: ADCSR,
    _reserved1: [u8; 0x04],
    #[doc = "0x06 - A/D Channel Select Register A1"]
    pub adansa1: ADANSA1,
    #[doc = "0x08 - A/D-Converted Value Addition/Average Channel Select Register 0"]
    pub adads0: ADADS0,
    #[doc = "0x0a - A/D-Converted Value Addition/Average Channel Select Register 1"]
    pub adads1: ADADS1,
    #[doc = "0x0c - A/D-Converted Value Addition/Average Count Select Register"]
    pub adadc: ADADC,
    _reserved5: [u8; 0x01],
    #[doc = "0x0e - A/D Control Extended Register"]
    pub adcer: ADCER,
    #[doc = "0x10 - A/D Conversion Start Trigger Select Register"]
    pub adstrgr: ADSTRGR,
    #[doc = "0x12 - A/D Conversion Extended Input Control Registers"]
    pub adexicr: ADEXICR,
    #[doc = "0x14 - A/D Channel Select Register B0"]
    pub adansb0: ADANSB0,
    #[doc = "0x16 - A/D Channel Select Register B1"]
    pub adansb1: ADANSB1,
    #[doc = "0x18 - A/D Data Duplexing Register"]
    pub addbldr: ADDBLDR,
    #[doc = "0x1a - A/D Temperature Sensor Data Register"]
    pub adtsdr: ADTSDR,
    #[doc = "0x1c - A/D Internal Reference Voltage Data Register"]
    pub adocdr: ADOCDR,
    #[doc = "0x1e - A/D Self-Diagnosis Data Register"]
    pub adrd: ADRD,
    #[doc = "0x20..0x26 - A/D Data Registers %s"]
    pub addr: [ADDR; 3],
    _reserved15: [u8; 0x1a],
    #[doc = "0x40 - A/D Data Registers %s"]
    pub addr16: ADDR16,
    #[doc = "0x42 - A/D Data Registers %s"]
    pub addr17: ADDR16,
    #[doc = "0x44 - A/D Data Registers %s"]
    pub addr18: ADDR16,
    #[doc = "0x46 - A/D Data Registers %s"]
    pub addr19: ADDR16,
    #[doc = "0x48 - A/D Data Registers %s"]
    pub addr20: ADDR16,
    #[doc = "0x4a - A/D Data Registers %s"]
    pub addr21: ADDR16,
    #[doc = "0x4c - A/D Data Registers %s"]
    pub addr22: ADDR16,
    _reserved22: [u8; 0x2c],
    #[doc = "0x7a - A/D Disconnection Detection Control Register"]
    pub addiscr: ADDISCR,
    _reserved23: [u8; 0x05],
    #[doc = "0x80 - A/D Group Scan Priority Control Register"]
    pub adgspcr: ADGSPCR,
    _reserved24: [u8; 0x02],
    #[doc = "0x84 - A/D Data Duplexing Register A"]
    pub addbldra: ADDBLDRA,
    #[doc = "0x86 - A/D Data Duplexing Register B"]
    pub addbldrb: ADDBLDRB,
    _reserved26: [u8; 0x04],
    #[doc = "0x8c - A/D Compare Function Window A/B Status Monitor Register"]
    pub adwinmon: ADWINMON,
    _reserved27: [u8; 0x03],
    #[doc = "0x90 - A/D Compare Function Control Register"]
    pub adcmpcr: ADCMPCR,
    #[doc = "0x92 - A/D Compare Function Window A Extended Input Select Register"]
    pub adcmpanser: ADCMPANSER,
    #[doc = "0x93 - A/D Compare Function Window A Extended Input Comparison Condition Setting Register"]
    pub adcmpler: ADCMPLER,
    #[doc = "0x94 - A/D Compare Function Window A Channel Select Register 0"]
    pub adcmpansr0: ADCMPANSR0,
    #[doc = "0x96 - A/D Compare Function Window A Channel Select Register 1"]
    pub adcmpansr1: ADCMPANSR1,
    #[doc = "0x98 - A/D Compare Function Window A Comparison Condition Setting Register 0"]
    pub adcmplr0: ADCMPLR0,
    #[doc = "0x9a - A/D Compare Function Window A Comparison Condition Setting Register 1"]
    pub adcmplr1: ADCMPLR1,
    #[doc = "0x9c - A/D Compare Function Window A Lower-Side/Upper-Side Level Setting Register"]
    pub adcmpdr: [ADCMPDR; 2],
    #[doc = "0xa0 - A/D Compare Function Window A Channel Status Register 0"]
    pub adcmpsr0: ADCMPSR0,
    #[doc = "0xa2 - A/D Compare Function Window A Channel Status Register1"]
    pub adcmpsr1: ADCMPSR1,
    #[doc = "0xa4 - A/D Compare Function Window A Extended Input Channel Status Register"]
    pub adcmpser: ADCMPSER,
    _reserved38: [u8; 0x01],
    #[doc = "0xa6 - A/D Compare Function Window B Channel Select Register"]
    pub adcmpbnsr: ADCMPBNSR,
    _reserved39: [u8; 0x01],
    #[doc = "0xa8 - A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register"]
    pub adwinllb: ADWINLLB,
    #[doc = "0xaa - A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register"]
    pub adwinulb: ADWINULB,
    #[doc = "0xac - A/D Compare Function Window B Status Register"]
    pub adcmpbsr: ADCMPBSR,
    _reserved42: [u8; 0x03],
    #[doc = "0xb0..0xd0 - A/D Data Buffer Registers %s"]
    pub adbuf: [ADBUF; 16],
    #[doc = "0xd0 - A/D Data Buffer Enable Register"]
    pub adbufen: ADBUFEN,
    _reserved44: [u8; 0x01],
    #[doc = "0xd2 - A/D Data Buffer Pointer Register"]
    pub adbufptr: ADBUFPTR,
    _reserved45: [u8; 0x0d],
    #[doc = "0xe0 - A/D Sampling State Register"]
    pub adsstr: [ADSSTR; 3],
}
#[doc = "ADCSR (rw) register accessor: an alias for `Reg<ADCSR_SPEC>`"]
pub type ADCSR = crate::Reg<adcsr::ADCSR_SPEC>;
#[doc = "A/D Control Register"]
pub mod adcsr;
#[doc = "ADANSA1 (rw) register accessor: an alias for `Reg<ADANSA1_SPEC>`"]
pub type ADANSA1 = crate::Reg<adansa1::ADANSA1_SPEC>;
#[doc = "A/D Channel Select Register A1"]
pub mod adansa1;
#[doc = "ADADS0 (rw) register accessor: an alias for `Reg<ADADS0_SPEC>`"]
pub type ADADS0 = crate::Reg<adads0::ADADS0_SPEC>;
#[doc = "A/D-Converted Value Addition/Average Channel Select Register 0"]
pub mod adads0;
#[doc = "ADADS1 (rw) register accessor: an alias for `Reg<ADADS1_SPEC>`"]
pub type ADADS1 = crate::Reg<adads1::ADADS1_SPEC>;
#[doc = "A/D-Converted Value Addition/Average Channel Select Register 1"]
pub mod adads1;
#[doc = "ADADC (rw) register accessor: an alias for `Reg<ADADC_SPEC>`"]
pub type ADADC = crate::Reg<adadc::ADADC_SPEC>;
#[doc = "A/D-Converted Value Addition/Average Count Select Register"]
pub mod adadc;
#[doc = "ADCER (rw) register accessor: an alias for `Reg<ADCER_SPEC>`"]
pub type ADCER = crate::Reg<adcer::ADCER_SPEC>;
#[doc = "A/D Control Extended Register"]
pub mod adcer;
#[doc = "ADSTRGR (rw) register accessor: an alias for `Reg<ADSTRGR_SPEC>`"]
pub type ADSTRGR = crate::Reg<adstrgr::ADSTRGR_SPEC>;
#[doc = "A/D Conversion Start Trigger Select Register"]
pub mod adstrgr;
#[doc = "ADEXICR (rw) register accessor: an alias for `Reg<ADEXICR_SPEC>`"]
pub type ADEXICR = crate::Reg<adexicr::ADEXICR_SPEC>;
#[doc = "A/D Conversion Extended Input Control Registers"]
pub mod adexicr;
#[doc = "ADANSB0 (rw) register accessor: an alias for `Reg<ADANSB0_SPEC>`"]
pub type ADANSB0 = crate::Reg<adansb0::ADANSB0_SPEC>;
#[doc = "A/D Channel Select Register B0"]
pub mod adansb0;
#[doc = "ADANSB1 (rw) register accessor: an alias for `Reg<ADANSB1_SPEC>`"]
pub type ADANSB1 = crate::Reg<adansb1::ADANSB1_SPEC>;
#[doc = "A/D Channel Select Register B1"]
pub mod adansb1;
#[doc = "ADDBLDR (r) register accessor: an alias for `Reg<ADDBLDR_SPEC>`"]
pub type ADDBLDR = crate::Reg<addbldr::ADDBLDR_SPEC>;
#[doc = "A/D Data Duplexing Register"]
pub mod addbldr;
#[doc = "ADTSDR (r) register accessor: an alias for `Reg<ADTSDR_SPEC>`"]
pub type ADTSDR = crate::Reg<adtsdr::ADTSDR_SPEC>;
#[doc = "A/D Temperature Sensor Data Register"]
pub mod adtsdr;
#[doc = "ADOCDR (r) register accessor: an alias for `Reg<ADOCDR_SPEC>`"]
pub type ADOCDR = crate::Reg<adocdr::ADOCDR_SPEC>;
#[doc = "A/D Internal Reference Voltage Data Register"]
pub mod adocdr;
#[doc = "ADRD (r) register accessor: an alias for `Reg<ADRD_SPEC>`"]
pub type ADRD = crate::Reg<adrd::ADRD_SPEC>;
#[doc = "A/D Self-Diagnosis Data Register"]
pub mod adrd;
#[doc = "ADDR (r) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "A/D Data Registers %s"]
pub mod addr;
pub use addr as addr16;
pub use ADDR as ADDR16;
#[doc = "ADDISCR (rw) register accessor: an alias for `Reg<ADDISCR_SPEC>`"]
pub type ADDISCR = crate::Reg<addiscr::ADDISCR_SPEC>;
#[doc = "A/D Disconnection Detection Control Register"]
pub mod addiscr;
#[doc = "ADGSPCR (rw) register accessor: an alias for `Reg<ADGSPCR_SPEC>`"]
pub type ADGSPCR = crate::Reg<adgspcr::ADGSPCR_SPEC>;
#[doc = "A/D Group Scan Priority Control Register"]
pub mod adgspcr;
#[doc = "ADDBLDRA (r) register accessor: an alias for `Reg<ADDBLDRA_SPEC>`"]
pub type ADDBLDRA = crate::Reg<addbldra::ADDBLDRA_SPEC>;
#[doc = "A/D Data Duplexing Register A"]
pub mod addbldra;
#[doc = "ADDBLDRB (r) register accessor: an alias for `Reg<ADDBLDRB_SPEC>`"]
pub type ADDBLDRB = crate::Reg<addbldrb::ADDBLDRB_SPEC>;
#[doc = "A/D Data Duplexing Register B"]
pub mod addbldrb;
#[doc = "ADWINMON (r) register accessor: an alias for `Reg<ADWINMON_SPEC>`"]
pub type ADWINMON = crate::Reg<adwinmon::ADWINMON_SPEC>;
#[doc = "A/D Compare Function Window A/B Status Monitor Register"]
pub mod adwinmon;
#[doc = "ADCMPCR (rw) register accessor: an alias for `Reg<ADCMPCR_SPEC>`"]
pub type ADCMPCR = crate::Reg<adcmpcr::ADCMPCR_SPEC>;
#[doc = "A/D Compare Function Control Register"]
pub mod adcmpcr;
#[doc = "ADCMPANSER (rw) register accessor: an alias for `Reg<ADCMPANSER_SPEC>`"]
pub type ADCMPANSER = crate::Reg<adcmpanser::ADCMPANSER_SPEC>;
#[doc = "A/D Compare Function Window A Extended Input Select Register"]
pub mod adcmpanser;
#[doc = "ADCMPLER (rw) register accessor: an alias for `Reg<ADCMPLER_SPEC>`"]
pub type ADCMPLER = crate::Reg<adcmpler::ADCMPLER_SPEC>;
#[doc = "A/D Compare Function Window A Extended Input Comparison Condition Setting Register"]
pub mod adcmpler;
#[doc = "ADCMPANSR0 (rw) register accessor: an alias for `Reg<ADCMPANSR0_SPEC>`"]
pub type ADCMPANSR0 = crate::Reg<adcmpansr0::ADCMPANSR0_SPEC>;
#[doc = "A/D Compare Function Window A Channel Select Register 0"]
pub mod adcmpansr0;
#[doc = "ADCMPANSR1 (rw) register accessor: an alias for `Reg<ADCMPANSR1_SPEC>`"]
pub type ADCMPANSR1 = crate::Reg<adcmpansr1::ADCMPANSR1_SPEC>;
#[doc = "A/D Compare Function Window A Channel Select Register 1"]
pub mod adcmpansr1;
#[doc = "ADCMPLR0 (rw) register accessor: an alias for `Reg<ADCMPLR0_SPEC>`"]
pub type ADCMPLR0 = crate::Reg<adcmplr0::ADCMPLR0_SPEC>;
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 0"]
pub mod adcmplr0;
#[doc = "ADCMPLR1 (rw) register accessor: an alias for `Reg<ADCMPLR1_SPEC>`"]
pub type ADCMPLR1 = crate::Reg<adcmplr1::ADCMPLR1_SPEC>;
#[doc = "A/D Compare Function Window A Comparison Condition Setting Register 1"]
pub mod adcmplr1;
#[doc = "ADCMPDR (rw) register accessor: an alias for `Reg<ADCMPDR_SPEC>`"]
pub type ADCMPDR = crate::Reg<adcmpdr::ADCMPDR_SPEC>;
#[doc = "A/D Compare Function Window A Lower-Side/Upper-Side Level Setting Register"]
pub mod adcmpdr;
#[doc = "ADCMPSR0 (rw) register accessor: an alias for `Reg<ADCMPSR0_SPEC>`"]
pub type ADCMPSR0 = crate::Reg<adcmpsr0::ADCMPSR0_SPEC>;
#[doc = "A/D Compare Function Window A Channel Status Register 0"]
pub mod adcmpsr0;
#[doc = "ADCMPSR1 (rw) register accessor: an alias for `Reg<ADCMPSR1_SPEC>`"]
pub type ADCMPSR1 = crate::Reg<adcmpsr1::ADCMPSR1_SPEC>;
#[doc = "A/D Compare Function Window A Channel Status Register1"]
pub mod adcmpsr1;
#[doc = "ADCMPSER (rw) register accessor: an alias for `Reg<ADCMPSER_SPEC>`"]
pub type ADCMPSER = crate::Reg<adcmpser::ADCMPSER_SPEC>;
#[doc = "A/D Compare Function Window A Extended Input Channel Status Register"]
pub mod adcmpser;
#[doc = "ADCMPBNSR (rw) register accessor: an alias for `Reg<ADCMPBNSR_SPEC>`"]
pub type ADCMPBNSR = crate::Reg<adcmpbnsr::ADCMPBNSR_SPEC>;
#[doc = "A/D Compare Function Window B Channel Select Register"]
pub mod adcmpbnsr;
#[doc = "ADWINLLB (rw) register accessor: an alias for `Reg<ADWINLLB_SPEC>`"]
pub type ADWINLLB = crate::Reg<adwinllb::ADWINLLB_SPEC>;
#[doc = "A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register"]
pub mod adwinllb;
#[doc = "ADWINULB (rw) register accessor: an alias for `Reg<ADWINULB_SPEC>`"]
pub type ADWINULB = crate::Reg<adwinulb::ADWINULB_SPEC>;
#[doc = "A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register"]
pub mod adwinulb;
#[doc = "ADCMPBSR (rw) register accessor: an alias for `Reg<ADCMPBSR_SPEC>`"]
pub type ADCMPBSR = crate::Reg<adcmpbsr::ADCMPBSR_SPEC>;
#[doc = "A/D Compare Function Window B Status Register"]
pub mod adcmpbsr;
#[doc = "ADBUF (r) register accessor: an alias for `Reg<ADBUF_SPEC>`"]
pub type ADBUF = crate::Reg<adbuf::ADBUF_SPEC>;
#[doc = "A/D Data Buffer Registers %s"]
pub mod adbuf;
#[doc = "ADBUFEN (rw) register accessor: an alias for `Reg<ADBUFEN_SPEC>`"]
pub type ADBUFEN = crate::Reg<adbufen::ADBUFEN_SPEC>;
#[doc = "A/D Data Buffer Enable Register"]
pub mod adbufen;
#[doc = "ADBUFPTR (rw) register accessor: an alias for `Reg<ADBUFPTR_SPEC>`"]
pub type ADBUFPTR = crate::Reg<adbufptr::ADBUFPTR_SPEC>;
#[doc = "A/D Data Buffer Pointer Register"]
pub mod adbufptr;
#[doc = "ADSSTR (rw) register accessor: an alias for `Reg<ADSSTR_SPEC>`"]
pub type ADSSTR = crate::Reg<adsstr::ADSSTR_SPEC>;
#[doc = "A/D Sampling State Register"]
pub mod adsstr;