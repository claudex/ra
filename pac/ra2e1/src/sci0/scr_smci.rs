#[doc = "Register `SCR_SMCI` reader"]
pub struct R(crate::R<SCR_SMCI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SMCI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SMCI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SMCI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR_SMCI` writer"]
pub struct W(crate::W<SCR_SMCI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SMCI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SCR_SMCI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SMCI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKE` reader - Clock Enable"]
pub type CKE_R = crate::FieldReader<u8, CKE_A>;
#[doc = "Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKE_A {
    #[doc = "0: When SMR_SMCI.GM = 0: Disable output The SCKn pin is available for use as an I/O port if set up in the I/O port settings When SMR_SMCI.GM = 1: Fix output low"]
    _00 = 0,
    #[doc = "1: When SMR_SMCI.GM = 0: Output clock When SMR_SMCI.GM = 1: Output clock"]
    _01 = 1,
    #[doc = "2: When SMR_SMCI.GM = 0: Setting prohibited When SMR_SMCI.GM = 1: Fix output high"]
    _10 = 2,
    #[doc = "3: When SMR_SMCI.GM = 0: Setting prohibited When SMR_SMCI.GM = 1: Output clock"]
    _11 = 3,
}
impl From<CKE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKE_A) -> Self {
        variant as _
    }
}
impl CKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKE_A {
        match self.bits {
            0 => CKE_A::_00,
            1 => CKE_A::_01,
            2 => CKE_A::_10,
            3 => CKE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CKE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CKE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CKE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CKE_A::_11
    }
}
#[doc = "Field `CKE` writer - Clock Enable"]
pub type CKE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SCR_SMCI_SPEC, u8, CKE_A, 2, O>;
impl<'a, const O: u8> CKE_W<'a, O> {
    #[doc = "When SMR_SMCI.GM = 0: Disable output The SCKn pin is available for use as an I/O port if set up in the I/O port settings When SMR_SMCI.GM = 1: Fix output low"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CKE_A::_00)
    }
    #[doc = "When SMR_SMCI.GM = 0: Output clock When SMR_SMCI.GM = 1: Output clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CKE_A::_01)
    }
    #[doc = "When SMR_SMCI.GM = 0: Setting prohibited When SMR_SMCI.GM = 1: Fix output high"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CKE_A::_10)
    }
    #[doc = "When SMR_SMCI.GM = 0: Setting prohibited When SMR_SMCI.GM = 1: Output clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CKE_A::_11)
    }
}
#[doc = "Field `TEIE` reader - Transmit End Interrupt Enable"]
pub type TEIE_R = crate::BitReader<bool>;
#[doc = "Field `TEIE` writer - Transmit End Interrupt Enable"]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCR_SMCI_SPEC, bool, O>;
#[doc = "Field `MPIE` reader - Multi-Processor Interrupt Enable"]
pub type MPIE_R = crate::BitReader<bool>;
#[doc = "Field `MPIE` writer - Multi-Processor Interrupt Enable"]
pub type MPIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCR_SMCI_SPEC, bool, O>;
#[doc = "Field `RE` reader - Receive Enable"]
pub type RE_R = crate::BitReader<RE_A>;
#[doc = "Receive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_A {
    #[doc = "0: Disable serial reception"]
    _0 = 0,
    #[doc = "1: Enable serial reception"]
    _1 = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::_0,
            true => RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RE_A::_1
    }
}
#[doc = "Field `RE` writer - Receive Enable"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCR_SMCI_SPEC, RE_A, O>;
impl<'a, const O: u8> RE_W<'a, O> {
    #[doc = "Disable serial reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RE_A::_0)
    }
    #[doc = "Enable serial reception"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RE_A::_1)
    }
}
#[doc = "Field `TE` reader - Transmit Enable"]
pub type TE_R = crate::BitReader<TE_A>;
#[doc = "Transmit Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TE_A {
    #[doc = "0: Disable serial transmission"]
    _0 = 0,
    #[doc = "1: Enable serial transmission"]
    _1 = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
impl TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::_0,
            true => TE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TE_A::_1
    }
}
#[doc = "Field `TE` writer - Transmit Enable"]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCR_SMCI_SPEC, TE_A, O>;
impl<'a, const O: u8> TE_W<'a, O> {
    #[doc = "Disable serial transmission"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TE_A::_0)
    }
    #[doc = "Enable serial transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TE_A::_1)
    }
}
#[doc = "Field `RIE` reader - Receive Interrupt Enable"]
pub type RIE_R = crate::BitReader<RIE_A>;
#[doc = "Receive Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIE_A {
    #[doc = "0: Disable SCIn_RXI and SCIn_ERI interrupt requests"]
    _0 = 0,
    #[doc = "1: Enable SCIn_RXI and SCIn_ERI interrupt requests"]
    _1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::_0,
            true => RIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
#[doc = "Field `RIE` writer - Receive Interrupt Enable"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCR_SMCI_SPEC, RIE_A, O>;
impl<'a, const O: u8> RIE_W<'a, O> {
    #[doc = "Disable SCIn_RXI and SCIn_ERI interrupt requests"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIE_A::_0)
    }
    #[doc = "Enable SCIn_RXI and SCIn_ERI interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIE_A::_1)
    }
}
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
pub type TIE_R = crate::BitReader<TIE_A>;
#[doc = "Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    #[doc = "0: Disable SCIn_TXI interrupt requests"]
    _0 = 0,
    #[doc = "1: Enable SCIn_TXI interrupt requests"]
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCR_SMCI_SPEC, TIE_A, O>;
impl<'a, const O: u8> TIE_W<'a, O> {
    #[doc = "Disable SCIn_TXI interrupt requests"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "Enable SCIn_TXI interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Enable"]
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Multi-Processor Interrupt Enable"]
    #[inline(always)]
    pub fn mpie(&self) -> MPIE_R {
        MPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Enable"]
    #[inline(always)]
    pub fn cke(&mut self) -> CKE_W<0> {
        CKE_W::new(self)
    }
    #[doc = "Bit 2 - Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<2> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 3 - Multi-Processor Interrupt Enable"]
    #[inline(always)]
    pub fn mpie(&mut self) -> MPIE_W<3> {
        MPIE_W::new(self)
    }
    #[doc = "Bit 4 - Receive Enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<4> {
        RE_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<5> {
        TE_W::new(self)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<6> {
        RIE_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<7> {
        TIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial Control Register for Smart Card Interface Mode (SCMR.SMIF = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr_smci](index.html) module"]
pub struct SCR_SMCI_SPEC;
impl crate::RegisterSpec for SCR_SMCI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scr_smci::R](R) reader structure"]
impl crate::Readable for SCR_SMCI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr_smci::W](W) writer structure"]
impl crate::Writable for SCR_SMCI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCR_SMCI to value 0"]
impl crate::Resettable for SCR_SMCI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
