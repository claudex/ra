#[doc = "Register `OSCSF` reader"]
pub struct R(crate::R<OSCSF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCSF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCSF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCSF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOCOSF` reader - HOCO Clock Oscillation Stabilization Flag"]
pub type HOCOSF_R = crate::BitReader<HOCOSF_A>;
#[doc = "HOCO Clock Oscillation Stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOCOSF_A {
    #[doc = "0: The HOCO clock is stopped or is not yet stable"]
    _0 = 0,
    #[doc = "1: The HOCO clock is stable, so is available for use as the system clock"]
    _1 = 1,
}
impl From<HOCOSF_A> for bool {
    #[inline(always)]
    fn from(variant: HOCOSF_A) -> Self {
        variant as u8 != 0
    }
}
impl HOCOSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOCOSF_A {
        match self.bits {
            false => HOCOSF_A::_0,
            true => HOCOSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HOCOSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HOCOSF_A::_1
    }
}
#[doc = "Field `MOSCSF` reader - Main Clock Oscillation Stabilization Flag"]
pub type MOSCSF_R = crate::BitReader<MOSCSF_A>;
#[doc = "Main Clock Oscillation Stabilization Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOSCSF_A {
    #[doc = "0: The main clock oscillator is stopped (MOSTP = 1) or is not yet stable"]
    _0 = 0,
    #[doc = "1: The main clock oscillator is stable, so is available for use as the system clock"]
    _1 = 1,
}
impl From<MOSCSF_A> for bool {
    #[inline(always)]
    fn from(variant: MOSCSF_A) -> Self {
        variant as u8 != 0
    }
}
impl MOSCSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOSCSF_A {
        match self.bits {
            false => MOSCSF_A::_0,
            true => MOSCSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOSCSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOSCSF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - HOCO Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn hocosf(&self) -> HOCOSF_R {
        HOCOSF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Main Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn moscsf(&self) -> MOSCSF_R {
        MOSCSF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Oscillation Stabilization Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscsf](index.html) module"]
pub struct OSCSF_SPEC;
impl crate::RegisterSpec for OSCSF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [oscsf::R](R) reader structure"]
impl crate::Readable for OSCSF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSCSF to value 0"]
impl crate::Resettable for OSCSF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
