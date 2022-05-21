#[doc = "Register `GPAFEN0` reader"]
pub struct R(crate::R<GPAFEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPAFEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPAFEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPAFEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPAFEN0` writer"]
pub struct W(crate::W<GPAFEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPAFEN0_SPEC>;
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
impl From<crate::W<GPAFEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPAFEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFEN0` reader - Async falling enabled 0"]
pub type AFEN0_R = crate::BitReader<bool>;
#[doc = "Field `AFEN0` writer - Async falling enabled 0"]
pub type AFEN0_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 0>;
#[doc = "Field `AFEN1` reader - Async falling enabled 1"]
pub type AFEN1_R = crate::BitReader<bool>;
#[doc = "Field `AFEN1` writer - Async falling enabled 1"]
pub type AFEN1_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 1>;
#[doc = "Field `AFEN2` reader - Async falling enabled 2"]
pub type AFEN2_R = crate::BitReader<bool>;
#[doc = "Field `AFEN2` writer - Async falling enabled 2"]
pub type AFEN2_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 2>;
#[doc = "Field `AFEN3` reader - Async falling enabled 3"]
pub type AFEN3_R = crate::BitReader<bool>;
#[doc = "Field `AFEN3` writer - Async falling enabled 3"]
pub type AFEN3_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 3>;
#[doc = "Field `AFEN4` reader - Async falling enabled 4"]
pub type AFEN4_R = crate::BitReader<bool>;
#[doc = "Field `AFEN4` writer - Async falling enabled 4"]
pub type AFEN4_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 4>;
#[doc = "Field `AFEN5` reader - Async falling enabled 5"]
pub type AFEN5_R = crate::BitReader<bool>;
#[doc = "Field `AFEN5` writer - Async falling enabled 5"]
pub type AFEN5_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 5>;
#[doc = "Field `AFEN6` reader - Async falling enabled 6"]
pub type AFEN6_R = crate::BitReader<bool>;
#[doc = "Field `AFEN6` writer - Async falling enabled 6"]
pub type AFEN6_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 6>;
#[doc = "Field `AFEN7` reader - Async falling enabled 7"]
pub type AFEN7_R = crate::BitReader<bool>;
#[doc = "Field `AFEN7` writer - Async falling enabled 7"]
pub type AFEN7_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 7>;
#[doc = "Field `AFEN8` reader - Async falling enabled 8"]
pub type AFEN8_R = crate::BitReader<bool>;
#[doc = "Field `AFEN8` writer - Async falling enabled 8"]
pub type AFEN8_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 8>;
#[doc = "Field `AFEN9` reader - Async falling enabled 9"]
pub type AFEN9_R = crate::BitReader<bool>;
#[doc = "Field `AFEN9` writer - Async falling enabled 9"]
pub type AFEN9_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 9>;
#[doc = "Field `AFEN10` reader - Async falling enabled 10"]
pub type AFEN10_R = crate::BitReader<bool>;
#[doc = "Field `AFEN10` writer - Async falling enabled 10"]
pub type AFEN10_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 10>;
#[doc = "Field `AFEN11` reader - Async falling enabled 11"]
pub type AFEN11_R = crate::BitReader<bool>;
#[doc = "Field `AFEN11` writer - Async falling enabled 11"]
pub type AFEN11_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 11>;
#[doc = "Field `AFEN12` reader - Async falling enabled 12"]
pub type AFEN12_R = crate::BitReader<bool>;
#[doc = "Field `AFEN12` writer - Async falling enabled 12"]
pub type AFEN12_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 12>;
#[doc = "Field `AFEN13` reader - Async falling enabled 13"]
pub type AFEN13_R = crate::BitReader<bool>;
#[doc = "Field `AFEN13` writer - Async falling enabled 13"]
pub type AFEN13_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 13>;
#[doc = "Field `AFEN14` reader - Async falling enabled 14"]
pub type AFEN14_R = crate::BitReader<bool>;
#[doc = "Field `AFEN14` writer - Async falling enabled 14"]
pub type AFEN14_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 14>;
#[doc = "Field `AFEN15` reader - Async falling enabled 15"]
pub type AFEN15_R = crate::BitReader<bool>;
#[doc = "Field `AFEN15` writer - Async falling enabled 15"]
pub type AFEN15_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 15>;
#[doc = "Field `AFEN16` reader - Async falling enabled 16"]
pub type AFEN16_R = crate::BitReader<bool>;
#[doc = "Field `AFEN16` writer - Async falling enabled 16"]
pub type AFEN16_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 16>;
#[doc = "Field `AFEN17` reader - Async falling enabled 17"]
pub type AFEN17_R = crate::BitReader<bool>;
#[doc = "Field `AFEN17` writer - Async falling enabled 17"]
pub type AFEN17_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 17>;
#[doc = "Field `AFEN18` reader - Async falling enabled 18"]
pub type AFEN18_R = crate::BitReader<bool>;
#[doc = "Field `AFEN18` writer - Async falling enabled 18"]
pub type AFEN18_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 18>;
#[doc = "Field `AFEN19` reader - Async falling enabled 19"]
pub type AFEN19_R = crate::BitReader<bool>;
#[doc = "Field `AFEN19` writer - Async falling enabled 19"]
pub type AFEN19_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 19>;
#[doc = "Field `AFEN20` reader - Async falling enabled 20"]
pub type AFEN20_R = crate::BitReader<bool>;
#[doc = "Field `AFEN20` writer - Async falling enabled 20"]
pub type AFEN20_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 20>;
#[doc = "Field `AFEN21` reader - Async falling enabled 21"]
pub type AFEN21_R = crate::BitReader<bool>;
#[doc = "Field `AFEN21` writer - Async falling enabled 21"]
pub type AFEN21_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 21>;
#[doc = "Field `AFEN22` reader - Async falling enabled 22"]
pub type AFEN22_R = crate::BitReader<bool>;
#[doc = "Field `AFEN22` writer - Async falling enabled 22"]
pub type AFEN22_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 22>;
#[doc = "Field `AFEN23` reader - Async falling enabled 23"]
pub type AFEN23_R = crate::BitReader<bool>;
#[doc = "Field `AFEN23` writer - Async falling enabled 23"]
pub type AFEN23_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 23>;
#[doc = "Field `AFEN24` reader - Async falling enabled 24"]
pub type AFEN24_R = crate::BitReader<bool>;
#[doc = "Field `AFEN24` writer - Async falling enabled 24"]
pub type AFEN24_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 24>;
#[doc = "Field `AFEN25` reader - Async falling enabled 25"]
pub type AFEN25_R = crate::BitReader<bool>;
#[doc = "Field `AFEN25` writer - Async falling enabled 25"]
pub type AFEN25_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 25>;
#[doc = "Field `AFEN26` reader - Async falling enabled 26"]
pub type AFEN26_R = crate::BitReader<bool>;
#[doc = "Field `AFEN26` writer - Async falling enabled 26"]
pub type AFEN26_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 26>;
#[doc = "Field `AFEN27` reader - Async falling enabled 27"]
pub type AFEN27_R = crate::BitReader<bool>;
#[doc = "Field `AFEN27` writer - Async falling enabled 27"]
pub type AFEN27_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 27>;
#[doc = "Field `AFEN28` reader - Async falling enabled 28"]
pub type AFEN28_R = crate::BitReader<bool>;
#[doc = "Field `AFEN28` writer - Async falling enabled 28"]
pub type AFEN28_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 28>;
#[doc = "Field `AFEN29` reader - Async falling enabled 29"]
pub type AFEN29_R = crate::BitReader<bool>;
#[doc = "Field `AFEN29` writer - Async falling enabled 29"]
pub type AFEN29_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 29>;
#[doc = "Field `AFEN30` reader - Async falling enabled 30"]
pub type AFEN30_R = crate::BitReader<bool>;
#[doc = "Field `AFEN30` writer - Async falling enabled 30"]
pub type AFEN30_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 30>;
#[doc = "Field `AFEN31` reader - Async falling enabled 31"]
pub type AFEN31_R = crate::BitReader<bool>;
#[doc = "Field `AFEN31` writer - Async falling enabled 31"]
pub type AFEN31_W<'a> = crate::BitWriter<'a, u32, GPAFEN0_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Async falling enabled 0"]
    #[inline(always)]
    pub fn afen0(&self) -> AFEN0_R {
        AFEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Async falling enabled 1"]
    #[inline(always)]
    pub fn afen1(&self) -> AFEN1_R {
        AFEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Async falling enabled 2"]
    #[inline(always)]
    pub fn afen2(&self) -> AFEN2_R {
        AFEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Async falling enabled 3"]
    #[inline(always)]
    pub fn afen3(&self) -> AFEN3_R {
        AFEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Async falling enabled 4"]
    #[inline(always)]
    pub fn afen4(&self) -> AFEN4_R {
        AFEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Async falling enabled 5"]
    #[inline(always)]
    pub fn afen5(&self) -> AFEN5_R {
        AFEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Async falling enabled 6"]
    #[inline(always)]
    pub fn afen6(&self) -> AFEN6_R {
        AFEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Async falling enabled 7"]
    #[inline(always)]
    pub fn afen7(&self) -> AFEN7_R {
        AFEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Async falling enabled 8"]
    #[inline(always)]
    pub fn afen8(&self) -> AFEN8_R {
        AFEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Async falling enabled 9"]
    #[inline(always)]
    pub fn afen9(&self) -> AFEN9_R {
        AFEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Async falling enabled 10"]
    #[inline(always)]
    pub fn afen10(&self) -> AFEN10_R {
        AFEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Async falling enabled 11"]
    #[inline(always)]
    pub fn afen11(&self) -> AFEN11_R {
        AFEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Async falling enabled 12"]
    #[inline(always)]
    pub fn afen12(&self) -> AFEN12_R {
        AFEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Async falling enabled 13"]
    #[inline(always)]
    pub fn afen13(&self) -> AFEN13_R {
        AFEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Async falling enabled 14"]
    #[inline(always)]
    pub fn afen14(&self) -> AFEN14_R {
        AFEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Async falling enabled 15"]
    #[inline(always)]
    pub fn afen15(&self) -> AFEN15_R {
        AFEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Async falling enabled 16"]
    #[inline(always)]
    pub fn afen16(&self) -> AFEN16_R {
        AFEN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Async falling enabled 17"]
    #[inline(always)]
    pub fn afen17(&self) -> AFEN17_R {
        AFEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Async falling enabled 18"]
    #[inline(always)]
    pub fn afen18(&self) -> AFEN18_R {
        AFEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Async falling enabled 19"]
    #[inline(always)]
    pub fn afen19(&self) -> AFEN19_R {
        AFEN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Async falling enabled 20"]
    #[inline(always)]
    pub fn afen20(&self) -> AFEN20_R {
        AFEN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Async falling enabled 21"]
    #[inline(always)]
    pub fn afen21(&self) -> AFEN21_R {
        AFEN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Async falling enabled 22"]
    #[inline(always)]
    pub fn afen22(&self) -> AFEN22_R {
        AFEN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Async falling enabled 23"]
    #[inline(always)]
    pub fn afen23(&self) -> AFEN23_R {
        AFEN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Async falling enabled 24"]
    #[inline(always)]
    pub fn afen24(&self) -> AFEN24_R {
        AFEN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Async falling enabled 25"]
    #[inline(always)]
    pub fn afen25(&self) -> AFEN25_R {
        AFEN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Async falling enabled 26"]
    #[inline(always)]
    pub fn afen26(&self) -> AFEN26_R {
        AFEN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Async falling enabled 27"]
    #[inline(always)]
    pub fn afen27(&self) -> AFEN27_R {
        AFEN27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Async falling enabled 28"]
    #[inline(always)]
    pub fn afen28(&self) -> AFEN28_R {
        AFEN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Async falling enabled 29"]
    #[inline(always)]
    pub fn afen29(&self) -> AFEN29_R {
        AFEN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Async falling enabled 30"]
    #[inline(always)]
    pub fn afen30(&self) -> AFEN30_R {
        AFEN30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Async falling enabled 31"]
    #[inline(always)]
    pub fn afen31(&self) -> AFEN31_R {
        AFEN31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Async falling enabled 0"]
    #[inline(always)]
    pub fn afen0(&mut self) -> AFEN0_W {
        AFEN0_W::new(self)
    }
    #[doc = "Bit 1 - Async falling enabled 1"]
    #[inline(always)]
    pub fn afen1(&mut self) -> AFEN1_W {
        AFEN1_W::new(self)
    }
    #[doc = "Bit 2 - Async falling enabled 2"]
    #[inline(always)]
    pub fn afen2(&mut self) -> AFEN2_W {
        AFEN2_W::new(self)
    }
    #[doc = "Bit 3 - Async falling enabled 3"]
    #[inline(always)]
    pub fn afen3(&mut self) -> AFEN3_W {
        AFEN3_W::new(self)
    }
    #[doc = "Bit 4 - Async falling enabled 4"]
    #[inline(always)]
    pub fn afen4(&mut self) -> AFEN4_W {
        AFEN4_W::new(self)
    }
    #[doc = "Bit 5 - Async falling enabled 5"]
    #[inline(always)]
    pub fn afen5(&mut self) -> AFEN5_W {
        AFEN5_W::new(self)
    }
    #[doc = "Bit 6 - Async falling enabled 6"]
    #[inline(always)]
    pub fn afen6(&mut self) -> AFEN6_W {
        AFEN6_W::new(self)
    }
    #[doc = "Bit 7 - Async falling enabled 7"]
    #[inline(always)]
    pub fn afen7(&mut self) -> AFEN7_W {
        AFEN7_W::new(self)
    }
    #[doc = "Bit 8 - Async falling enabled 8"]
    #[inline(always)]
    pub fn afen8(&mut self) -> AFEN8_W {
        AFEN8_W::new(self)
    }
    #[doc = "Bit 9 - Async falling enabled 9"]
    #[inline(always)]
    pub fn afen9(&mut self) -> AFEN9_W {
        AFEN9_W::new(self)
    }
    #[doc = "Bit 10 - Async falling enabled 10"]
    #[inline(always)]
    pub fn afen10(&mut self) -> AFEN10_W {
        AFEN10_W::new(self)
    }
    #[doc = "Bit 11 - Async falling enabled 11"]
    #[inline(always)]
    pub fn afen11(&mut self) -> AFEN11_W {
        AFEN11_W::new(self)
    }
    #[doc = "Bit 12 - Async falling enabled 12"]
    #[inline(always)]
    pub fn afen12(&mut self) -> AFEN12_W {
        AFEN12_W::new(self)
    }
    #[doc = "Bit 13 - Async falling enabled 13"]
    #[inline(always)]
    pub fn afen13(&mut self) -> AFEN13_W {
        AFEN13_W::new(self)
    }
    #[doc = "Bit 14 - Async falling enabled 14"]
    #[inline(always)]
    pub fn afen14(&mut self) -> AFEN14_W {
        AFEN14_W::new(self)
    }
    #[doc = "Bit 15 - Async falling enabled 15"]
    #[inline(always)]
    pub fn afen15(&mut self) -> AFEN15_W {
        AFEN15_W::new(self)
    }
    #[doc = "Bit 16 - Async falling enabled 16"]
    #[inline(always)]
    pub fn afen16(&mut self) -> AFEN16_W {
        AFEN16_W::new(self)
    }
    #[doc = "Bit 17 - Async falling enabled 17"]
    #[inline(always)]
    pub fn afen17(&mut self) -> AFEN17_W {
        AFEN17_W::new(self)
    }
    #[doc = "Bit 18 - Async falling enabled 18"]
    #[inline(always)]
    pub fn afen18(&mut self) -> AFEN18_W {
        AFEN18_W::new(self)
    }
    #[doc = "Bit 19 - Async falling enabled 19"]
    #[inline(always)]
    pub fn afen19(&mut self) -> AFEN19_W {
        AFEN19_W::new(self)
    }
    #[doc = "Bit 20 - Async falling enabled 20"]
    #[inline(always)]
    pub fn afen20(&mut self) -> AFEN20_W {
        AFEN20_W::new(self)
    }
    #[doc = "Bit 21 - Async falling enabled 21"]
    #[inline(always)]
    pub fn afen21(&mut self) -> AFEN21_W {
        AFEN21_W::new(self)
    }
    #[doc = "Bit 22 - Async falling enabled 22"]
    #[inline(always)]
    pub fn afen22(&mut self) -> AFEN22_W {
        AFEN22_W::new(self)
    }
    #[doc = "Bit 23 - Async falling enabled 23"]
    #[inline(always)]
    pub fn afen23(&mut self) -> AFEN23_W {
        AFEN23_W::new(self)
    }
    #[doc = "Bit 24 - Async falling enabled 24"]
    #[inline(always)]
    pub fn afen24(&mut self) -> AFEN24_W {
        AFEN24_W::new(self)
    }
    #[doc = "Bit 25 - Async falling enabled 25"]
    #[inline(always)]
    pub fn afen25(&mut self) -> AFEN25_W {
        AFEN25_W::new(self)
    }
    #[doc = "Bit 26 - Async falling enabled 26"]
    #[inline(always)]
    pub fn afen26(&mut self) -> AFEN26_W {
        AFEN26_W::new(self)
    }
    #[doc = "Bit 27 - Async falling enabled 27"]
    #[inline(always)]
    pub fn afen27(&mut self) -> AFEN27_W {
        AFEN27_W::new(self)
    }
    #[doc = "Bit 28 - Async falling enabled 28"]
    #[inline(always)]
    pub fn afen28(&mut self) -> AFEN28_W {
        AFEN28_W::new(self)
    }
    #[doc = "Bit 29 - Async falling enabled 29"]
    #[inline(always)]
    pub fn afen29(&mut self) -> AFEN29_W {
        AFEN29_W::new(self)
    }
    #[doc = "Bit 30 - Async falling enabled 30"]
    #[inline(always)]
    pub fn afen30(&mut self) -> AFEN30_W {
        AFEN30_W::new(self)
    }
    #[doc = "Bit 31 - Async falling enabled 31"]
    #[inline(always)]
    pub fn afen31(&mut self) -> AFEN31_W {
        AFEN31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Async. Falling Edge Detect 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpafen0](index.html) module"]
pub struct GPAFEN0_SPEC;
impl crate::RegisterSpec for GPAFEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpafen0::R](R) reader structure"]
impl crate::Readable for GPAFEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpafen0::W](W) writer structure"]
impl crate::Writable for GPAFEN0_SPEC {
    type Writer = W;
}
