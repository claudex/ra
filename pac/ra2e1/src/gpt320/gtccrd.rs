#[doc = "Register `GTCCRD` reader"]
pub struct R(crate::R<GTCCRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTCCRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTCCRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTCCRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTCCRD` writer"]
pub struct W(crate::W<GTCCRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTCCRD_SPEC>;
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
impl From<crate::W<GTCCRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTCCRD_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Compare Capture Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtccrd](index.html) module"]
pub struct GTCCRD_SPEC;
impl crate::RegisterSpec for GTCCRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtccrd::R](R) reader structure"]
impl crate::Readable for GTCCRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtccrd::W](W) writer structure"]
impl crate::Writable for GTCCRD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GTCCRD to value 0xffff_ffff"]
impl crate::Resettable for GTCCRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
