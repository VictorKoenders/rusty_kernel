#[doc = "Register `DOEPINT` reader"]
pub struct R(crate::R<DOEPINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPINT` writer"]
pub struct W(crate::W<DOEPINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPINT_SPEC>;
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
impl From<crate::W<DOEPINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRC` reader - Transfer completed interrupt"]
pub type XFRC_R = crate::BitReader<bool>;
#[doc = "Field `XFRC` writer - Transfer completed interrupt"]
pub type XFRC_W<'a> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, 0>;
#[doc = "Field `EPDISD` reader - Endpoint disabled interrupt"]
pub type EPDISD_R = crate::BitReader<bool>;
#[doc = "Field `EPDISD` writer - Endpoint disabled interrupt"]
pub type EPDISD_W<'a> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, 1>;
#[doc = "Field `STUP` reader - SETUP phase done"]
pub type STUP_R = crate::BitReader<bool>;
#[doc = "Field `STUP` writer - SETUP phase done"]
pub type STUP_W<'a> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, 3>;
#[doc = "Field `OTEPDIS` reader - OUT token received when endpoint disabled"]
pub type OTEPDIS_R = crate::BitReader<bool>;
#[doc = "Field `OTEPDIS` writer - OUT token received when endpoint disabled"]
pub type OTEPDIS_W<'a> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, 4>;
#[doc = "Field `B2BSTUP` reader - Back-to-back SETUP packets received"]
pub type B2BSTUP_R = crate::BitReader<bool>;
#[doc = "Field `B2BSTUP` writer - Back-to-back SETUP packets received"]
pub type B2BSTUP_W<'a> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, 6>;
#[doc = "Field `NYET` reader - NYET interrupt"]
pub type NYET_R = crate::BitReader<bool>;
#[doc = "Field `NYET` writer - NYET interrupt"]
pub type NYET_W<'a> = crate::BitWriter<'a, u32, DOEPINT_SPEC, bool, 14>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W {
        XFRC_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W {
        EPDISD_W::new(self)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    pub fn stup(&mut self) -> STUP_W {
        STUP_W::new(self)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    pub fn otepdis(&mut self) -> OTEPDIS_W {
        OTEPDIS_W::new(self)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2BSTUP_W {
        B2BSTUP_W::new(self)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W {
        NYET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint](index.html) module"]
pub struct DOEPINT_SPEC;
impl crate::RegisterSpec for DOEPINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepint::R](R) reader structure"]
impl crate::Readable for DOEPINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepint::W](W) writer structure"]
impl crate::Writable for DOEPINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPINT to value 0x80"]
impl crate::Resettable for DOEPINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
