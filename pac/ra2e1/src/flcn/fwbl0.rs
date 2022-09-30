#[doc = "Register `FWBL0` reader"]
pub struct R(crate::R<FWBL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWBL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWBL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWBL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWBL0` writer"]
pub struct W(crate::W<FWBL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWBL0_SPEC>;
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
impl From<crate::W<FWBL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWBL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDATA` reader - Flash Write Buffer L0"]
pub type WDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDATA` writer - Flash Write Buffer L0"]
pub type WDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FWBL0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Flash Write Buffer L0"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Flash Write Buffer L0"]
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W<0> {
        WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Write Buffer Register L0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwbl0](index.html) module"]
pub struct FWBL0_SPEC;
impl crate::RegisterSpec for FWBL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fwbl0::R](R) reader structure"]
impl crate::Readable for FWBL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwbl0::W](W) writer structure"]
impl crate::Writable for FWBL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FWBL0 to value 0"]
impl crate::Resettable for FWBL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
