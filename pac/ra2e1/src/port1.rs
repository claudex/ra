#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_pdr: [u8; 0x04],
    _reserved_1_eidr: [u8; 0x04],
    _reserved_2_porr: [u8; 0x04],
    _reserved_3_eorr: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Port Control Register 1"]
    #[inline(always)]
    pub fn podr(&self) -> &PODR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const PODR) }
    }
    #[doc = "0x00 - Port Control Register 1"]
    #[inline(always)]
    pub fn pcntr1(&self) -> &PCNTR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const PCNTR1) }
    }
    #[doc = "0x02 - Port Control Register 1"]
    #[inline(always)]
    pub fn pdr(&self) -> &PDR {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const PDR) }
    }
    #[doc = "0x04 - Port Control Register 2"]
    #[inline(always)]
    pub fn eidr(&self) -> &EIDR {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const EIDR) }
    }
    #[doc = "0x04 - Port Control Register 2"]
    #[inline(always)]
    pub fn pcntr2(&self) -> &PCNTR2 {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const PCNTR2) }
    }
    #[doc = "0x06 - Port Control Register 2"]
    #[inline(always)]
    pub fn pidr(&self) -> &PIDR {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const PIDR) }
    }
    #[doc = "0x08 - Port Control Register 3"]
    #[inline(always)]
    pub fn porr(&self) -> &PORR {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const PORR) }
    }
    #[doc = "0x08 - Port Control Register 3"]
    #[inline(always)]
    pub fn pcntr3(&self) -> &PCNTR3 {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const PCNTR3) }
    }
    #[doc = "0x0a - Port Control Register 3"]
    #[inline(always)]
    pub fn posr(&self) -> &POSR {
        unsafe { &*(((self as *const Self) as *const u8).add(10usize) as *const POSR) }
    }
    #[doc = "0x0c - Port Control Register 4"]
    #[inline(always)]
    pub fn eorr(&self) -> &EORR {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const EORR) }
    }
    #[doc = "0x0c - Port Control Register 4"]
    #[inline(always)]
    pub fn pcntr4(&self) -> &PCNTR4 {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const PCNTR4) }
    }
    #[doc = "0x0e - Port Control Register 4"]
    #[inline(always)]
    pub fn eosr(&self) -> &EOSR {
        unsafe { &*(((self as *const Self) as *const u8).add(14usize) as *const EOSR) }
    }
}
#[doc = "PCNTR1 (rw) register accessor: an alias for `Reg<PCNTR1_SPEC>`"]
pub type PCNTR1 = crate::Reg<pcntr1::PCNTR1_SPEC>;
#[doc = "Port Control Register 1"]
pub mod pcntr1;
#[doc = "PODR (rw) register accessor: an alias for `Reg<PODR_SPEC>`"]
pub type PODR = crate::Reg<podr::PODR_SPEC>;
#[doc = "Port Control Register 1"]
pub mod podr;
#[doc = "PDR (rw) register accessor: an alias for `Reg<PDR_SPEC>`"]
pub type PDR = crate::Reg<pdr::PDR_SPEC>;
#[doc = "Port Control Register 1"]
pub mod pdr;
#[doc = "PCNTR2 (r) register accessor: an alias for `Reg<PCNTR2_SPEC>`"]
pub type PCNTR2 = crate::Reg<pcntr2::PCNTR2_SPEC>;
#[doc = "Port Control Register 2"]
pub mod pcntr2;
#[doc = "EIDR (r) register accessor: an alias for `Reg<EIDR_SPEC>`"]
pub type EIDR = crate::Reg<eidr::EIDR_SPEC>;
#[doc = "Port Control Register 2"]
pub mod eidr;
#[doc = "PIDR (r) register accessor: an alias for `Reg<PIDR_SPEC>`"]
pub type PIDR = crate::Reg<pidr::PIDR_SPEC>;
#[doc = "Port Control Register 2"]
pub mod pidr;
#[doc = "PCNTR3 (w) register accessor: an alias for `Reg<PCNTR3_SPEC>`"]
pub type PCNTR3 = crate::Reg<pcntr3::PCNTR3_SPEC>;
#[doc = "Port Control Register 3"]
pub mod pcntr3;
#[doc = "PORR (w) register accessor: an alias for `Reg<PORR_SPEC>`"]
pub type PORR = crate::Reg<porr::PORR_SPEC>;
#[doc = "Port Control Register 3"]
pub mod porr;
#[doc = "POSR (w) register accessor: an alias for `Reg<POSR_SPEC>`"]
pub type POSR = crate::Reg<posr::POSR_SPEC>;
#[doc = "Port Control Register 3"]
pub mod posr;
#[doc = "PCNTR4 (rw) register accessor: an alias for `Reg<PCNTR4_SPEC>`"]
pub type PCNTR4 = crate::Reg<pcntr4::PCNTR4_SPEC>;
#[doc = "Port Control Register 4"]
pub mod pcntr4;
#[doc = "EORR (rw) register accessor: an alias for `Reg<EORR_SPEC>`"]
pub type EORR = crate::Reg<eorr::EORR_SPEC>;
#[doc = "Port Control Register 4"]
pub mod eorr;
#[doc = "EOSR (rw) register accessor: an alias for `Reg<EOSR_SPEC>`"]
pub type EOSR = crate::Reg<eosr::EOSR_SPEC>;
#[doc = "Port Control Register 4"]
pub mod eosr;
