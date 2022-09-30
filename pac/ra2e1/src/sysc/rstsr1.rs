#[doc = "Register `RSTSR1` reader"]
pub struct R(crate::R<RSTSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTSR1` writer"]
pub struct W(crate::W<RSTSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTSR1_SPEC>;
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
impl From<crate::W<RSTSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IWDTRF` reader - Independent Watchdog Timer Reset Detect Flag"]
pub type IWDTRF_R = crate::BitReader<IWDTRF_A>;
#[doc = "Independent Watchdog Timer Reset Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWDTRF_A {
    #[doc = "0: Independent watchdog timer reset not detected"]
    _0 = 0,
    #[doc = "1: Independent watchdog timer reset detected"]
    _1 = 1,
}
impl From<IWDTRF_A> for bool {
    #[inline(always)]
    fn from(variant: IWDTRF_A) -> Self {
        variant as u8 != 0
    }
}
impl IWDTRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDTRF_A {
        match self.bits {
            false => IWDTRF_A::_0,
            true => IWDTRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IWDTRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IWDTRF_A::_1
    }
}
#[doc = "Field `IWDTRF` writer - Independent Watchdog Timer Reset Detect Flag"]
pub type IWDTRF_W<'a, const O: u8> = crate::BitWriter<'a, u16, RSTSR1_SPEC, IWDTRF_A, O>;
impl<'a, const O: u8> IWDTRF_W<'a, O> {
    #[doc = "Independent watchdog timer reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IWDTRF_A::_0)
    }
    #[doc = "Independent watchdog timer reset detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IWDTRF_A::_1)
    }
}
#[doc = "Field `WDTRF` reader - Watchdog Timer Reset Detect Flag"]
pub type WDTRF_R = crate::BitReader<WDTRF_A>;
#[doc = "Watchdog Timer Reset Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTRF_A {
    #[doc = "0: Watchdog timer reset not detected"]
    _0 = 0,
    #[doc = "1: Watchdog timer reset detected"]
    _1 = 1,
}
impl From<WDTRF_A> for bool {
    #[inline(always)]
    fn from(variant: WDTRF_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTRF_A {
        match self.bits {
            false => WDTRF_A::_0,
            true => WDTRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WDTRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WDTRF_A::_1
    }
}
#[doc = "Field `WDTRF` writer - Watchdog Timer Reset Detect Flag"]
pub type WDTRF_W<'a, const O: u8> = crate::BitWriter<'a, u16, RSTSR1_SPEC, WDTRF_A, O>;
impl<'a, const O: u8> WDTRF_W<'a, O> {
    #[doc = "Watchdog timer reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDTRF_A::_0)
    }
    #[doc = "Watchdog timer reset detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDTRF_A::_1)
    }
}
#[doc = "Field `SWRF` reader - Software Reset Detect Flag"]
pub type SWRF_R = crate::BitReader<SWRF_A>;
#[doc = "Software Reset Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRF_A {
    #[doc = "0: Software reset not detected"]
    _0 = 0,
    #[doc = "1: Software reset detected"]
    _1 = 1,
}
impl From<SWRF_A> for bool {
    #[inline(always)]
    fn from(variant: SWRF_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRF_A {
        match self.bits {
            false => SWRF_A::_0,
            true => SWRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWRF_A::_1
    }
}
#[doc = "Field `SWRF` writer - Software Reset Detect Flag"]
pub type SWRF_W<'a, const O: u8> = crate::BitWriter<'a, u16, RSTSR1_SPEC, SWRF_A, O>;
impl<'a, const O: u8> SWRF_W<'a, O> {
    #[doc = "Software reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRF_A::_0)
    }
    #[doc = "Software reset detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRF_A::_1)
    }
}
#[doc = "Field `RPERF` reader - SRAM Parity Error Reset Detect Flag"]
pub type RPERF_R = crate::BitReader<RPERF_A>;
#[doc = "SRAM Parity Error Reset Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPERF_A {
    #[doc = "0: SRAM parity error reset not detected"]
    _0 = 0,
    #[doc = "1: SRAM parity error reset detected"]
    _1 = 1,
}
impl From<RPERF_A> for bool {
    #[inline(always)]
    fn from(variant: RPERF_A) -> Self {
        variant as u8 != 0
    }
}
impl RPERF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPERF_A {
        match self.bits {
            false => RPERF_A::_0,
            true => RPERF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPERF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPERF_A::_1
    }
}
#[doc = "Field `RPERF` writer - SRAM Parity Error Reset Detect Flag"]
pub type RPERF_W<'a, const O: u8> = crate::BitWriter<'a, u16, RSTSR1_SPEC, RPERF_A, O>;
impl<'a, const O: u8> RPERF_W<'a, O> {
    #[doc = "SRAM parity error reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPERF_A::_0)
    }
    #[doc = "SRAM parity error reset detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPERF_A::_1)
    }
}
#[doc = "Field `BUSSRF` reader - Bus Slave MPU Error Reset Detect Flag"]
pub type BUSSRF_R = crate::BitReader<BUSSRF_A>;
#[doc = "Bus Slave MPU Error Reset Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSSRF_A {
    #[doc = "0: Bus slave MPU error reset not detected"]
    _0 = 0,
    #[doc = "1: Bus slave MPU error reset detected"]
    _1 = 1,
}
impl From<BUSSRF_A> for bool {
    #[inline(always)]
    fn from(variant: BUSSRF_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSSRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSSRF_A {
        match self.bits {
            false => BUSSRF_A::_0,
            true => BUSSRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSSRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSSRF_A::_1
    }
}
#[doc = "Field `BUSSRF` writer - Bus Slave MPU Error Reset Detect Flag"]
pub type BUSSRF_W<'a, const O: u8> = crate::BitWriter<'a, u16, RSTSR1_SPEC, BUSSRF_A, O>;
impl<'a, const O: u8> BUSSRF_W<'a, O> {
    #[doc = "Bus slave MPU error reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSSRF_A::_0)
    }
    #[doc = "Bus slave MPU error reset detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSSRF_A::_1)
    }
}
#[doc = "Field `BUSMRF` reader - Bus Master MPU Error Reset Detect Flag"]
pub type BUSMRF_R = crate::BitReader<BUSMRF_A>;
#[doc = "Bus Master MPU Error Reset Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSMRF_A {
    #[doc = "0: Bus master MPU error reset not detected"]
    _0 = 0,
    #[doc = "1: Bus master MPU error reset detected"]
    _1 = 1,
}
impl From<BUSMRF_A> for bool {
    #[inline(always)]
    fn from(variant: BUSMRF_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSMRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSMRF_A {
        match self.bits {
            false => BUSMRF_A::_0,
            true => BUSMRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSMRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSMRF_A::_1
    }
}
#[doc = "Field `BUSMRF` writer - Bus Master MPU Error Reset Detect Flag"]
pub type BUSMRF_W<'a, const O: u8> = crate::BitWriter<'a, u16, RSTSR1_SPEC, BUSMRF_A, O>;
impl<'a, const O: u8> BUSMRF_W<'a, O> {
    #[doc = "Bus master MPU error reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUSMRF_A::_0)
    }
    #[doc = "Bus master MPU error reset detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUSMRF_A::_1)
    }
}
#[doc = "Field `SPERF` reader - CPU Stack Pointer Error Reset Detect Flag"]
pub type SPERF_R = crate::BitReader<SPERF_A>;
#[doc = "CPU Stack Pointer Error Reset Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPERF_A {
    #[doc = "0: CPU stack pointer error reset not detected"]
    _0 = 0,
    #[doc = "1: CPU stack pointer error reset detected"]
    _1 = 1,
}
impl From<SPERF_A> for bool {
    #[inline(always)]
    fn from(variant: SPERF_A) -> Self {
        variant as u8 != 0
    }
}
impl SPERF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPERF_A {
        match self.bits {
            false => SPERF_A::_0,
            true => SPERF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPERF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPERF_A::_1
    }
}
#[doc = "Field `SPERF` writer - CPU Stack Pointer Error Reset Detect Flag"]
pub type SPERF_W<'a, const O: u8> = crate::BitWriter<'a, u16, RSTSR1_SPEC, SPERF_A, O>;
impl<'a, const O: u8> SPERF_W<'a, O> {
    #[doc = "CPU stack pointer error reset not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPERF_A::_0)
    }
    #[doc = "CPU stack pointer error reset detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPERF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Independent Watchdog Timer Reset Detect Flag"]
    #[inline(always)]
    pub fn iwdtrf(&self) -> IWDTRF_R {
        IWDTRF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer Reset Detect Flag"]
    #[inline(always)]
    pub fn wdtrf(&self) -> WDTRF_R {
        WDTRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset Detect Flag"]
    #[inline(always)]
    pub fn swrf(&self) -> SWRF_R {
        SWRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM Parity Error Reset Detect Flag"]
    #[inline(always)]
    pub fn rperf(&self) -> RPERF_R {
        RPERF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus Slave MPU Error Reset Detect Flag"]
    #[inline(always)]
    pub fn bussrf(&self) -> BUSSRF_R {
        BUSSRF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus Master MPU Error Reset Detect Flag"]
    #[inline(always)]
    pub fn busmrf(&self) -> BUSMRF_R {
        BUSMRF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU Stack Pointer Error Reset Detect Flag"]
    #[inline(always)]
    pub fn sperf(&self) -> SPERF_R {
        SPERF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Independent Watchdog Timer Reset Detect Flag"]
    #[inline(always)]
    pub fn iwdtrf(&mut self) -> IWDTRF_W<0> {
        IWDTRF_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog Timer Reset Detect Flag"]
    #[inline(always)]
    pub fn wdtrf(&mut self) -> WDTRF_W<1> {
        WDTRF_W::new(self)
    }
    #[doc = "Bit 2 - Software Reset Detect Flag"]
    #[inline(always)]
    pub fn swrf(&mut self) -> SWRF_W<2> {
        SWRF_W::new(self)
    }
    #[doc = "Bit 8 - SRAM Parity Error Reset Detect Flag"]
    #[inline(always)]
    pub fn rperf(&mut self) -> RPERF_W<8> {
        RPERF_W::new(self)
    }
    #[doc = "Bit 10 - Bus Slave MPU Error Reset Detect Flag"]
    #[inline(always)]
    pub fn bussrf(&mut self) -> BUSSRF_W<10> {
        BUSSRF_W::new(self)
    }
    #[doc = "Bit 11 - Bus Master MPU Error Reset Detect Flag"]
    #[inline(always)]
    pub fn busmrf(&mut self) -> BUSMRF_W<11> {
        BUSMRF_W::new(self)
    }
    #[doc = "Bit 12 - CPU Stack Pointer Error Reset Detect Flag"]
    #[inline(always)]
    pub fn sperf(&mut self) -> SPERF_W<12> {
        SPERF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstsr1](index.html) module"]
pub struct RSTSR1_SPEC;
impl crate::RegisterSpec for RSTSR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rstsr1::R](R) reader structure"]
impl crate::Readable for RSTSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstsr1::W](W) writer structure"]
impl crate::Writable for RSTSR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTSR1 to value 0"]
impl crate::Resettable for RSTSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
