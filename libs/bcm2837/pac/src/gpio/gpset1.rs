#[doc = "Register `GPSET1` writer"]
pub struct W(crate::W<GPSET1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPSET1_SPEC>;
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
impl From<crate::W<GPSET1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPSET1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET32` writer - Set 32"]
pub type SET32_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 0>;
#[doc = "Field `SET33` writer - Set 33"]
pub type SET33_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 1>;
#[doc = "Field `SET34` writer - Set 34"]
pub type SET34_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 2>;
#[doc = "Field `SET35` writer - Set 35"]
pub type SET35_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 3>;
#[doc = "Field `SET36` writer - Set 36"]
pub type SET36_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 4>;
#[doc = "Field `SET37` writer - Set 37"]
pub type SET37_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 5>;
#[doc = "Field `SET38` writer - Set 38"]
pub type SET38_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 6>;
#[doc = "Field `SET39` writer - Set 39"]
pub type SET39_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 7>;
#[doc = "Field `SET40` writer - Set 40"]
pub type SET40_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 8>;
#[doc = "Field `SET41` writer - Set 41"]
pub type SET41_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 9>;
#[doc = "Field `SET42` writer - Set 42"]
pub type SET42_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 10>;
#[doc = "Field `SET43` writer - Set 43"]
pub type SET43_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 11>;
#[doc = "Field `SET44` writer - Set 44"]
pub type SET44_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 12>;
#[doc = "Field `SET45` writer - Set 45"]
pub type SET45_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 13>;
#[doc = "Field `SET46` writer - Set 46"]
pub type SET46_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 14>;
#[doc = "Field `SET47` writer - Set 47"]
pub type SET47_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 15>;
#[doc = "Field `SET48` writer - Set 48"]
pub type SET48_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 16>;
#[doc = "Field `SET49` writer - Set 49"]
pub type SET49_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 17>;
#[doc = "Field `SET50` writer - Set 50"]
pub type SET50_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 18>;
#[doc = "Field `SET51` writer - Set 51"]
pub type SET51_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 19>;
#[doc = "Field `SET52` writer - Set 52"]
pub type SET52_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 20>;
#[doc = "Field `SET53` writer - Set 53"]
pub type SET53_W<'a> = crate::BitWriter1S<'a, u32, GPSET1_SPEC, bool, 21>;
impl W {
    #[doc = "Bit 0 - Set 32"]
    #[inline(always)]
    pub fn set32(&mut self) -> SET32_W {
        SET32_W::new(self)
    }
    #[doc = "Bit 1 - Set 33"]
    #[inline(always)]
    pub fn set33(&mut self) -> SET33_W {
        SET33_W::new(self)
    }
    #[doc = "Bit 2 - Set 34"]
    #[inline(always)]
    pub fn set34(&mut self) -> SET34_W {
        SET34_W::new(self)
    }
    #[doc = "Bit 3 - Set 35"]
    #[inline(always)]
    pub fn set35(&mut self) -> SET35_W {
        SET35_W::new(self)
    }
    #[doc = "Bit 4 - Set 36"]
    #[inline(always)]
    pub fn set36(&mut self) -> SET36_W {
        SET36_W::new(self)
    }
    #[doc = "Bit 5 - Set 37"]
    #[inline(always)]
    pub fn set37(&mut self) -> SET37_W {
        SET37_W::new(self)
    }
    #[doc = "Bit 6 - Set 38"]
    #[inline(always)]
    pub fn set38(&mut self) -> SET38_W {
        SET38_W::new(self)
    }
    #[doc = "Bit 7 - Set 39"]
    #[inline(always)]
    pub fn set39(&mut self) -> SET39_W {
        SET39_W::new(self)
    }
    #[doc = "Bit 8 - Set 40"]
    #[inline(always)]
    pub fn set40(&mut self) -> SET40_W {
        SET40_W::new(self)
    }
    #[doc = "Bit 9 - Set 41"]
    #[inline(always)]
    pub fn set41(&mut self) -> SET41_W {
        SET41_W::new(self)
    }
    #[doc = "Bit 10 - Set 42"]
    #[inline(always)]
    pub fn set42(&mut self) -> SET42_W {
        SET42_W::new(self)
    }
    #[doc = "Bit 11 - Set 43"]
    #[inline(always)]
    pub fn set43(&mut self) -> SET43_W {
        SET43_W::new(self)
    }
    #[doc = "Bit 12 - Set 44"]
    #[inline(always)]
    pub fn set44(&mut self) -> SET44_W {
        SET44_W::new(self)
    }
    #[doc = "Bit 13 - Set 45"]
    #[inline(always)]
    pub fn set45(&mut self) -> SET45_W {
        SET45_W::new(self)
    }
    #[doc = "Bit 14 - Set 46"]
    #[inline(always)]
    pub fn set46(&mut self) -> SET46_W {
        SET46_W::new(self)
    }
    #[doc = "Bit 15 - Set 47"]
    #[inline(always)]
    pub fn set47(&mut self) -> SET47_W {
        SET47_W::new(self)
    }
    #[doc = "Bit 16 - Set 48"]
    #[inline(always)]
    pub fn set48(&mut self) -> SET48_W {
        SET48_W::new(self)
    }
    #[doc = "Bit 17 - Set 49"]
    #[inline(always)]
    pub fn set49(&mut self) -> SET49_W {
        SET49_W::new(self)
    }
    #[doc = "Bit 18 - Set 50"]
    #[inline(always)]
    pub fn set50(&mut self) -> SET50_W {
        SET50_W::new(self)
    }
    #[doc = "Bit 19 - Set 51"]
    #[inline(always)]
    pub fn set51(&mut self) -> SET51_W {
        SET51_W::new(self)
    }
    #[doc = "Bit 20 - Set 52"]
    #[inline(always)]
    pub fn set52(&mut self) -> SET52_W {
        SET52_W::new(self)
    }
    #[doc = "Bit 21 - Set 53"]
    #[inline(always)]
    pub fn set53(&mut self) -> SET53_W {
        SET53_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Output Set 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpset1](index.html) module"]
pub struct GPSET1_SPEC;
impl crate::RegisterSpec for GPSET1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpset1::W](W) writer structure"]
impl crate::Writable for GPSET1_SPEC {
    type Writer = W;
}
