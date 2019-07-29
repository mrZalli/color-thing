use std::fmt;
use std::marker::PhantomData;

use crate::*;

/// A HSV color
///
/// ## Type arguments
/// `H` is the type of hue channel, `T` is the type of the saturation and value channels.
///
/// `S` is this color's colorspace.
#[derive(Debug, PartialOrd, PartialEq)]
pub struct HSVColor<H, T, S> {
    pub h: H,
    pub s: T,
    pub v: T,
    _space: PhantomData<S>
}

impl<H, T, S> HSVColor<H, T, S> {
    /// Deconstructs this color into a tuple of it's channels
    #[inline]
    pub fn tuple(self) -> (H, T, T) {
        (self.h, self.s, self.v)
    }
}

impl<H, T, S> HSVColor<H, T, S>
    where Self: Color
{
    /// Create a new HSV value.
    ///
    /// The value is normalized on creation.
    pub fn new(h: H, s: T, v: T) -> Self {
        HSVColor { h, s, v, _space: PhantomData }.normalize()
    }

}

impl<H: Channel, T: Channel, S> HSVColor<H, T, S> {
    /// Transform this color into RGB form
    ///
    /// This should be done to a normalized HSV color.
    pub fn rgb(self) -> RGBColor<T, S> {
        let h = cuwtf(self.h.conv::<AngleDeg<f32>>()) / 60.0;
        let (s, v) = (cuwtf(self.s), cuwtf(self.v));

        // largest, second largest and the smallest component
        let c = s * v;
        let x = c * (1.0 - (h % 2.0 - 1.0).abs());
        let min = v - c;

        let (r, g, b) =
            match h as u8 {
                0   => (  c,   x, 0.0),
                1   => (  x,   c, 0.0),
                2   => (0.0,   c,   x),
                3   => (0.0,   x,   c),
                4   => (  x, 0.0,   c),
                5|6 => (  c, 0.0,   x),
                _   => panic!("Invalid hue value: {:?}", h)
            };

        (cuwf::<T>(r + min),
         cuwf::<T>(g + min),
         cuwf::<T>(b + min)).into()
    }

    #[inline]
    pub fn conv<H2: Channel, T2: Channel>(self) -> HSVColor<H2, T2, S> {
        HSVColor { h: self.h.conv(), s: self.s.conv(), v: self.v.conv(), _space: PhantomData }
    }
}

impl<H: Channel, T: Channel, S> Color for HSVColor<H, T, S>
    where Self: Clone
{
    /// Normalize the color's values by normalizing the hue and zeroing the unnecessary channels
    ///
    /// If value channel is zero, black is returned.
    /// If saturation channel is zero, hue is set to zero.
    ///
    /// Otherwise the color itself is returned, with it's channels put to their proper ranges
    fn normalize(self) -> Self {
        let (h, s, v) = self.tuple();
        if v == T::ch_zero() { Self::default() }
        else if s == T::ch_zero() {
            HSVColor {
                h: H::ch_zero(),
                s: T::ch_zero(),
                v: v.to_range(),
                _space: PhantomData
            }
        } else {
            HSVColor {
                h: h.to_range(),
                s: s.to_range(),
                v: v.to_range(),
                _space: PhantomData
            }
        }
    }

    fn is_normal(&self) -> bool {
        let (h, s, v) = self.clone().tuple();
        let (h0, t0) = (H::ch_zero(), T::ch_zero());

        if !h.in_range() || !s.in_range() || !v.in_range() {
            false
        } else if v == t0 {
            // color black
            if h == h0 && s == t0 { true }
            else { false }
        } else if s == t0 {
            // a grey color
            if h == h0 { true }
            else { false }
        } else { true }
    }
}

impl<H: Channel, T: Channel> From<BaseColor> for HSVColor<H, T, SRGBSpace>
    where Self: Color
{
    #[inline]
    fn from(base_color: BaseColor) -> Self {
        use self::BaseColor::*;

        let f = |h: f32, s: f32, v: f32|
            Self::new(AngleDeg(h).conv(), s.conv(), v.conv());

        match base_color {
            Black   => f(  0.0, 0.0, 0.0),
            Grey    => f(  0.0, 0.0, 0.5),
            White   => f(  0.0, 0.0, 1.0),
            Red     => f(  0.0, 1.0, 1.0),
            Yellow  => f( 60.0, 1.0, 1.0),
            Green   => f(120.0, 1.0, 1.0),
            Cyan    => f(180.0, 1.0, 1.0),
            Blue    => f(240.0, 1.0, 1.0),
            Magenta => f(300.0, 1.0, 1.0),
        }
    }
}

impl<H: Channel, T: Channel> From<BaseColor> for HSVColor<H, T, LinearSpace> {
    #[inline]
    fn from(base_color: BaseColor) -> Self {
        RGBColor::<f32, LinearSpace>::from(base_color).hsv().conv()
    }
}

impl<H: Channel, T: Channel, S> From<(H, T, T)> for HSVColor<H, T, S>
    where Self: Color
{
    fn from(tuple: (H, T, T)) -> Self {
        let (h, s, v) = tuple;
        HSVColor::new(h, s, v)
    }
}

impl<H: Clone + Channel, T: Clone + Channel, S> From<&(H, T, T)> for HSVColor<H, T, S>
    where Self: Color
{
    fn from(tuple: &(H, T, T)) -> Self {
        let (h, s, v) = tuple.clone();
        HSVColor::new(h, s, v)
    }
}

impl<H: Channel, T: Channel, S> Default for HSVColor<H, T, S> {
    fn default() -> Self {
        HSVColor {
            h: H::ch_zero(),
            s: T::ch_zero(),
            v: T::ch_zero(),
            _space: PhantomData
        }
    }
}

impl<H: Clone, T: Clone, S> Clone for HSVColor<H, T, S> {
    fn clone(&self) -> Self {
        HSVColor {
            h: self.h.clone(),
            s: self.s.clone(),
            v: self.v.clone(),
            _space: PhantomData
        }
    }
}

impl<H: Copy, T: Copy, S> Copy for HSVColor<H, T, S> {}

// TODO make more generic
impl<S> fmt::Display for HSVColor<f32, f32, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>5.1}°,{:>5.1}%,{:>5.1}%", self.h, self.s * 100.0, self.v * 100.0)
    }
}
