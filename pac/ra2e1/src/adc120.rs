#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - A/D Control Register"]
    pub adcsr: ADCSR,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - A/D Channel Select Register A0"]
    pub adansa0: ADANSA0,
    #[doc = "0x06 - A/D Channel Select Register A1"]
    pub adansa1: ADANSA1,
    #[doc = "0x08 - A/D-Converted Value Addition/Average Channel Select Register 0"]
    pub adads0: ADADS0,
    #[doc = "0x0a - A/D-Converted Value Addition/Average Channel Select Register 1"]
    pub adads1: ADADS1,
    #[doc = "0x0c - A/D-Converted Value Addition/Average Count Select Register"]
    pub adadc: ADADC,
    _reserved6: [u8; 0x01],
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
    #[doc = "0x20 - A/D Data Registers 0"]
    pub addr0: ADDR0,
    #[doc = "0x22 - A/D Data Registers 1"]
    pub addr1: ADDR1,
    #[doc = "0x24 - A/D Data Registers 2"]
    pub addr2: ADDR2,
    #[doc = "0x26 - A/D Data Registers 3"]
    pub addr3: ADDR3,
    #[doc = "0x28 - A/D Data Registers 4"]
    pub addr4: ADDR4,
    #[doc = "0x2a - A/D Data Registers 5"]
    pub addr5: ADDR5,
    #[doc = "0x2c - A/D Data Registers 6"]
    pub addr6: ADDR6,
    #[doc = "0x2e - A/D Data Registers 7"]
    pub addr7: ADDR7,
    #[doc = "0x30 - A/D Data Registers 8"]
    pub addr8: ADDR8,
    #[doc = "0x32 - A/D Data Registers 9"]
    pub addr9: ADDR9,
    #[doc = "0x34 - A/D Data Registers 10"]
    pub addr10: ADDR10,
    _reserved26: [u8; 0x0a],
    #[doc = "0x40 - A/D CTSU TSCAP Voltage Data Register"]
    pub adctdr: ADCTDR,
    #[doc = "0x42 - A/D Data Registers 17"]
    pub addr17: ADDR17,
    #[doc = "0x44 - A/D Data Registers 18"]
    pub addr18: ADDR18,
    #[doc = "0x46 - A/D Data Registers 19"]
    pub addr19: ADDR19,
    #[doc = "0x48 - A/D Data Registers 20"]
    pub addr20: ADDR20,
    #[doc = "0x4a - A/D Data Registers 21"]
    pub addr21: ADDR21,
    #[doc = "0x4c - A/D Data Registers 22"]
    pub addr22: ADDR22,
    _reserved33: [u8; 0x2c],
    #[doc = "0x7a - A/D Disconnection Detection Control Register"]
    pub addiscr: ADDISCR,
    _reserved34: [u8; 0x03],
    #[doc = "0x7e - A/D Conversion Operation Mode Select Register"]
    pub adacsr: ADACSR,
    _reserved35: [u8; 0x01],
    #[doc = "0x80 - A/D Group Scan Priority Control Register"]
    pub adgspcr: ADGSPCR,
    _reserved36: [u8; 0x02],
    #[doc = "0x84 - A/D Data Duplexing Register A"]
    pub addbldra: ADDBLDRA,
    #[doc = "0x86 - A/D Data Duplexing Register B"]
    pub addbldrb: ADDBLDRB,
    _reserved38: [u8; 0x02],
    #[doc = "0x8a - A/D High-Potential/Low-Potential Reference Voltage Control Register"]
    pub adhvrefcnt: ADHVREFCNT,
    _reserved39: [u8; 0x01],
    #[doc = "0x8c - A/D Compare Function Window A/B Status Monitor Register"]
    pub adwinmon: ADWINMON,
    _reserved40: [u8; 0x03],
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
    _reserved51: [u8; 0x01],
    #[doc = "0xa6 - A/D Compare Function Window B Channel Select Register"]
    pub adcmpbnsr: ADCMPBNSR,
    _reserved52: [u8; 0x01],
    #[doc = "0xa8 - A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register"]
    pub adwinllb: ADWINLLB,
    #[doc = "0xaa - A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register"]
    pub adwinulb: ADWINULB,
    #[doc = "0xac - A/D Compare Function Window B Status Register"]
    pub adcmpbsr: ADCMPBSR,
    _reserved55: [u8; 0x30],
    #[doc = "0xdd - A/D Sampling State Register"]
    pub adsstrl: ADSSTRL,
    #[doc = "0xde - A/D Sampling State Register"]
    pub adsstrt: ADSSTRT,
    #[doc = "0xdf - A/D Sampling State Register"]
    pub adsstro: ADSSTRO,
    #[doc = "0xe0..0xeb - A/D Sampling State Register"]
    pub adsstr: [ADSSTR; 11],
}
#[doc = "ADCSR (rw) register accessor: an alias for `Reg<ADCSR_SPEC>`"]
pub type ADCSR = crate::Reg<adcsr::ADCSR_SPEC>;
#[doc = "A/D Control Register"]
pub mod adcsr;
#[doc = "ADANSA0 (rw) register accessor: an alias for `Reg<ADANSA0_SPEC>`"]
pub type ADANSA0 = crate::Reg<adansa0::ADANSA0_SPEC>;
#[doc = "A/D Channel Select Register A0"]
pub mod adansa0;
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
#[doc = "ADCTDR (r) register accessor: an alias for `Reg<ADCTDR_SPEC>`"]
pub type ADCTDR = crate::Reg<adctdr::ADCTDR_SPEC>;
#[doc = "A/D CTSU TSCAP Voltage Data Register"]
pub mod adctdr;
#[doc = "ADDISCR (rw) register accessor: an alias for `Reg<ADDISCR_SPEC>`"]
pub type ADDISCR = crate::Reg<addiscr::ADDISCR_SPEC>;
#[doc = "A/D Disconnection Detection Control Register"]
pub mod addiscr;
#[doc = "ADACSR (rw) register accessor: an alias for `Reg<ADACSR_SPEC>`"]
pub type ADACSR = crate::Reg<adacsr::ADACSR_SPEC>;
#[doc = "A/D Conversion Operation Mode Select Register"]
pub mod adacsr;
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
#[doc = "ADHVREFCNT (rw) register accessor: an alias for `Reg<ADHVREFCNT_SPEC>`"]
pub type ADHVREFCNT = crate::Reg<adhvrefcnt::ADHVREFCNT_SPEC>;
#[doc = "A/D High-Potential/Low-Potential Reference Voltage Control Register"]
pub mod adhvrefcnt;
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
#[doc = "ADSSTRL (rw) register accessor: an alias for `Reg<ADSSTRL_SPEC>`"]
pub type ADSSTRL = crate::Reg<adsstrl::ADSSTRL_SPEC>;
#[doc = "A/D Sampling State Register"]
pub mod adsstrl;
#[doc = "ADSSTRT (rw) register accessor: an alias for `Reg<ADSSTRT_SPEC>`"]
pub type ADSSTRT = crate::Reg<adsstrt::ADSSTRT_SPEC>;
#[doc = "A/D Sampling State Register"]
pub mod adsstrt;
#[doc = "ADSSTRO (rw) register accessor: an alias for `Reg<ADSSTRO_SPEC>`"]
pub type ADSSTRO = crate::Reg<adsstro::ADSSTRO_SPEC>;
#[doc = "A/D Sampling State Register"]
pub mod adsstro;
#[doc = "ADSSTR (rw) register accessor: an alias for `Reg<ADSSTR_SPEC>`"]
pub type ADSSTR = crate::Reg<adsstr::ADSSTR_SPEC>;
#[doc = "A/D Sampling State Register"]
pub mod adsstr;
#[doc = "ADDR0 (r) register accessor: an alias for `Reg<ADDR0_SPEC>`"]
pub type ADDR0 = crate::Reg<addr0::ADDR0_SPEC>;
#[doc = "A/D Data Registers 0"]
pub mod addr0;
#[doc = "ADDR1 (r) register accessor: an alias for `Reg<ADDR1_SPEC>`"]
pub type ADDR1 = crate::Reg<addr1::ADDR1_SPEC>;
#[doc = "A/D Data Registers 1"]
pub mod addr1;
#[doc = "ADDR2 (r) register accessor: an alias for `Reg<ADDR2_SPEC>`"]
pub type ADDR2 = crate::Reg<addr2::ADDR2_SPEC>;
#[doc = "A/D Data Registers 2"]
pub mod addr2;
#[doc = "ADDR3 (r) register accessor: an alias for `Reg<ADDR3_SPEC>`"]
pub type ADDR3 = crate::Reg<addr3::ADDR3_SPEC>;
#[doc = "A/D Data Registers 3"]
pub mod addr3;
#[doc = "ADDR4 (r) register accessor: an alias for `Reg<ADDR4_SPEC>`"]
pub type ADDR4 = crate::Reg<addr4::ADDR4_SPEC>;
#[doc = "A/D Data Registers 4"]
pub mod addr4;
#[doc = "ADDR5 (r) register accessor: an alias for `Reg<ADDR5_SPEC>`"]
pub type ADDR5 = crate::Reg<addr5::ADDR5_SPEC>;
#[doc = "A/D Data Registers 5"]
pub mod addr5;
#[doc = "ADDR6 (r) register accessor: an alias for `Reg<ADDR6_SPEC>`"]
pub type ADDR6 = crate::Reg<addr6::ADDR6_SPEC>;
#[doc = "A/D Data Registers 6"]
pub mod addr6;
#[doc = "ADDR7 (r) register accessor: an alias for `Reg<ADDR7_SPEC>`"]
pub type ADDR7 = crate::Reg<addr7::ADDR7_SPEC>;
#[doc = "A/D Data Registers 7"]
pub mod addr7;
#[doc = "ADDR8 (r) register accessor: an alias for `Reg<ADDR8_SPEC>`"]
pub type ADDR8 = crate::Reg<addr8::ADDR8_SPEC>;
#[doc = "A/D Data Registers 8"]
pub mod addr8;
#[doc = "ADDR9 (r) register accessor: an alias for `Reg<ADDR9_SPEC>`"]
pub type ADDR9 = crate::Reg<addr9::ADDR9_SPEC>;
#[doc = "A/D Data Registers 9"]
pub mod addr9;
#[doc = "ADDR10 (r) register accessor: an alias for `Reg<ADDR10_SPEC>`"]
pub type ADDR10 = crate::Reg<addr10::ADDR10_SPEC>;
#[doc = "A/D Data Registers 10"]
pub mod addr10;
#[doc = "ADDR17 (r) register accessor: an alias for `Reg<ADDR17_SPEC>`"]
pub type ADDR17 = crate::Reg<addr17::ADDR17_SPEC>;
#[doc = "A/D Data Registers 17"]
pub mod addr17;
#[doc = "ADDR18 (r) register accessor: an alias for `Reg<ADDR18_SPEC>`"]
pub type ADDR18 = crate::Reg<addr18::ADDR18_SPEC>;
#[doc = "A/D Data Registers 18"]
pub mod addr18;
#[doc = "ADDR19 (r) register accessor: an alias for `Reg<ADDR19_SPEC>`"]
pub type ADDR19 = crate::Reg<addr19::ADDR19_SPEC>;
#[doc = "A/D Data Registers 19"]
pub mod addr19;
#[doc = "ADDR20 (r) register accessor: an alias for `Reg<ADDR20_SPEC>`"]
pub type ADDR20 = crate::Reg<addr20::ADDR20_SPEC>;
#[doc = "A/D Data Registers 20"]
pub mod addr20;
#[doc = "ADDR21 (r) register accessor: an alias for `Reg<ADDR21_SPEC>`"]
pub type ADDR21 = crate::Reg<addr21::ADDR21_SPEC>;
#[doc = "A/D Data Registers 21"]
pub mod addr21;
#[doc = "ADDR22 (r) register accessor: an alias for `Reg<ADDR22_SPEC>`"]
pub type ADDR22 = crate::Reg<addr22::ADDR22_SPEC>;
#[doc = "A/D Data Registers 22"]
pub mod addr22;
