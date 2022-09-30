#[doc = "Register `R64CNT` reader"]
pub struct R(crate::R<R64CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R64CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R64CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R64CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F64HZ` reader - 64-Hz Flag"]
pub type F64HZ_R = crate::BitReader<bool>;
#[doc = "Field `F32HZ` reader - 32-Hz Flag"]
pub type F32HZ_R = crate::BitReader<bool>;
#[doc = "Field `F16HZ` reader - 16-Hz Flag"]
pub type F16HZ_R = crate::BitReader<bool>;
#[doc = "Field `F8HZ` reader - 8-Hz Flag"]
pub type F8HZ_R = crate::BitReader<bool>;
#[doc = "Field `F4HZ` reader - 4-Hz Flag"]
pub type F4HZ_R = crate::BitReader<bool>;
#[doc = "Field `F2HZ` reader - 2-Hz Flag"]
pub type F2HZ_R = crate::BitReader<bool>;
#[doc = "Field `F1HZ` reader - 1-Hz Flag"]
pub type F1HZ_R = crate::BitReader<bool>;
#[doc = "Field `R64OVF` reader - "]
pub type R64OVF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - 64-Hz Flag"]
    #[inline(always)]
    pub fn f64hz(&self) -> F64HZ_R {
        F64HZ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 32-Hz Flag"]
    #[inline(always)]
    pub fn f32hz(&self) -> F32HZ_R {
        F32HZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 16-Hz Flag"]
    #[inline(always)]
    pub fn f16hz(&self) -> F16HZ_R {
        F16HZ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 8-Hz Flag"]
    #[inline(always)]
    pub fn f8hz(&self) -> F8HZ_R {
        F8HZ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4-Hz Flag"]
    #[inline(always)]
    pub fn f4hz(&self) -> F4HZ_R {
        F4HZ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 2-Hz Flag"]
    #[inline(always)]
    pub fn f2hz(&self) -> F2HZ_R {
        F2HZ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1-Hz Flag"]
    #[inline(always)]
    pub fn f1hz(&self) -> F1HZ_R {
        F1HZ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn r64ovf(&self) -> R64OVF_R {
        R64OVF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "64-Hz Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r64cnt](index.html) module"]
pub struct R64CNT_SPEC;
impl crate::RegisterSpec for R64CNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [r64cnt::R](R) reader structure"]
impl crate::Readable for R64CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R64CNT to value 0"]
impl crate::Resettable for R64CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
