#[doc = "Register `LVDSAR` reader"]
pub struct R(crate::R<LVDSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVDSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVDSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVDSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVDSAR` writer"]
pub struct W(crate::W<LVDSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVDSAR_SPEC>;
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
impl From<crate::W<LVDSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVDSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONSEC0` reader - Non Secure Attribute bit 0"]
pub type NONSEC0_R = crate::BitReader<NONSEC0_A>;
#[doc = "Non Secure Attribute bit 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC0_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC0_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC0_A {
        match self.bits {
            false => NONSEC0_A::_0,
            true => NONSEC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC0_A::_1
    }
}
#[doc = "Field `NONSEC0` writer - Non Secure Attribute bit 0"]
pub type NONSEC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LVDSAR_SPEC, NONSEC0_A, O>;
impl<'a, const O: u8> NONSEC0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC0_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC0_A::_1)
    }
}
#[doc = "Field `NONSEC1` reader - Non Secure Attribute bit 1"]
pub type NONSEC1_R = crate::BitReader<NONSEC1_A>;
#[doc = "Non Secure Attribute bit 1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONSEC1_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non Secure"]
    _1 = 1,
}
impl From<NONSEC1_A> for bool {
    #[inline(always)]
    fn from(variant: NONSEC1_A) -> Self {
        variant as u8 != 0
    }
}
impl NONSEC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONSEC1_A {
        match self.bits {
            false => NONSEC1_A::_0,
            true => NONSEC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NONSEC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NONSEC1_A::_1
    }
}
#[doc = "Field `NONSEC1` writer - Non Secure Attribute bit 1"]
pub type NONSEC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LVDSAR_SPEC, NONSEC1_A, O>;
impl<'a, const O: u8> NONSEC1_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NONSEC1_A::_0)
    }
    #[doc = "Non Secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NONSEC1_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Non Secure Attribute bit 0"]
    #[inline(always)]
    pub fn nonsec0(&self) -> NONSEC0_R {
        NONSEC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non Secure Attribute bit 1"]
    #[inline(always)]
    pub fn nonsec1(&self) -> NONSEC1_R {
        NONSEC1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non Secure Attribute bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec0(&mut self) -> NONSEC0_W<0> {
        NONSEC0_W::new(self)
    }
    #[doc = "Bit 1 - Non Secure Attribute bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec1(&mut self) -> NONSEC1_W<1> {
        NONSEC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Voltage Detection Security Attribution Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvdsar](index.html) module"]
pub struct LVDSAR_SPEC;
impl crate::RegisterSpec for LVDSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lvdsar::R](R) reader structure"]
impl crate::Readable for LVDSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvdsar::W](W) writer structure"]
impl crate::Writable for LVDSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVDSAR to value 0xffff_ffff"]
impl crate::Resettable for LVDSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}