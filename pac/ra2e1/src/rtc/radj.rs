#[doc = "Register `RADJ` reader"]
pub struct R(crate::R<RADJ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RADJ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RADJ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RADJ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RADJ` writer"]
pub struct W(crate::W<RADJ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RADJ_SPEC>;
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
impl From<crate::W<RADJ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RADJ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADJ` reader - Adjustment Value"]
pub type ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADJ` writer - Adjustment Value"]
pub type ADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RADJ_SPEC, u8, u8, 6, O>;
#[doc = "Field `PMADJ` reader - Plus-Minus"]
pub type PMADJ_R = crate::FieldReader<u8, PMADJ_A>;
#[doc = "Plus-Minus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PMADJ_A {
    #[doc = "0: Do not perform adjustment."]
    _00 = 0,
    #[doc = "1: In normal operation mode, adjustment is performed by the addition to the prescaler. In low-consumption clock mode, adjustment is performed by the addition to the 64-Hz counter."]
    _01 = 1,
    #[doc = "2: In normal operation mode, adjustment is performed by the subtraction from the prescaler. In low-consumption clock mode, adjustment is performed by the subtraction from the 64-Hz counter."]
    _10 = 2,
    #[doc = "3: Setting prohibited."]
    _11 = 3,
}
impl From<PMADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: PMADJ_A) -> Self {
        variant as _
    }
}
impl PMADJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMADJ_A {
        match self.bits {
            0 => PMADJ_A::_00,
            1 => PMADJ_A::_01,
            2 => PMADJ_A::_10,
            3 => PMADJ_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PMADJ_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PMADJ_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PMADJ_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PMADJ_A::_11
    }
}
#[doc = "Field `PMADJ` writer - Plus-Minus"]
pub type PMADJ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, RADJ_SPEC, u8, PMADJ_A, 2, O>;
impl<'a, const O: u8> PMADJ_W<'a, O> {
    #[doc = "Do not perform adjustment."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PMADJ_A::_00)
    }
    #[doc = "In normal operation mode, adjustment is performed by the addition to the prescaler. In low-consumption clock mode, adjustment is performed by the addition to the 64-Hz counter."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PMADJ_A::_01)
    }
    #[doc = "In normal operation mode, adjustment is performed by the subtraction from the prescaler. In low-consumption clock mode, adjustment is performed by the subtraction from the 64-Hz counter."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PMADJ_A::_10)
    }
    #[doc = "Setting prohibited."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PMADJ_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:5 - Adjustment Value"]
    #[inline(always)]
    pub fn adj(&self) -> ADJ_R {
        ADJ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Plus-Minus"]
    #[inline(always)]
    pub fn pmadj(&self) -> PMADJ_R {
        PMADJ_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Adjustment Value"]
    #[inline(always)]
    pub fn adj(&mut self) -> ADJ_W<0> {
        ADJ_W::new(self)
    }
    #[doc = "Bits 6:7 - Plus-Minus"]
    #[inline(always)]
    pub fn pmadj(&mut self) -> PMADJ_W<6> {
        PMADJ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time Error Adjustment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [radj](index.html) module"]
pub struct RADJ_SPEC;
impl crate::RegisterSpec for RADJ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [radj::R](R) reader structure"]
impl crate::Readable for RADJ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [radj::W](W) writer structure"]
impl crate::Writable for RADJ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RADJ to value 0"]
impl crate::Resettable for RADJ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
