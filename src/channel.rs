pub mod angle;

use num_traits::NumCast;

use crate::{cuwf, cuwtf};

/// A trait for color channels
pub trait Channel: Sized + PartialOrd + NumCast {
    /// Tells whether this is a channel with integer value
    ///
    /// If false the channel has a floating point value.
    const INTEGER: bool;

    /// The maximum value for this channel, inclusive
    ///
    /// With integers it's usually the max value, with floats it's one.
    fn ch_max() -> Self;

    /// The middle value for this channel
    ///
    /// Half of `ch_max`
    fn ch_mid() -> Self;

    /// The zero value for this channel
    fn ch_zero() -> Self;

    /// Takes this channel value and converts it into any other channel type
    ///
    /// The channel's range is taken into account, eg. 1.0 in f32 is converted into 255 in u8.
    ///
    /// The values will be made to fit into their range.
    fn conv<T: Channel>(self) -> T {
        let float = cuwtf(self.clamp()) / cuwtf(Self::ch_max()) * cuwtf(T::ch_max());
        cuwf(if T::INTEGER { float.round() } else { float })
    }

    /// Return whether this value is inside the channel's allowed range
    fn in_range(&self) -> bool {
        (self <= &Self::ch_max()) && (self >= &Self::ch_zero())
    }

    /// Returns this value clamped to channel's range
    fn clamp(self) -> Self {
        if self > Self::ch_max() {
            Self::ch_max()
        } else if self < Self::ch_zero() {
            Self::ch_zero()
        } else {
            self
        }
    }
}

macro_rules! impl_uint_channels {
    ( $( $type:ty ),* ) => { $(
        impl Channel for $type {
            const INTEGER: bool = true;
            fn ch_max() -> Self { <$type>::max_value() }
            fn ch_mid() -> Self { <$type>::max_value() / 2 }
            fn ch_zero() -> Self { 0 }
        }
    )* };
}

impl_uint_channels!(u8, u16, u32);

impl Channel for f32 {
    const INTEGER: bool = false;
    fn ch_max() -> Self {
        1.0
    }
    fn ch_mid() -> Self {
        0.5
    }
    fn ch_zero() -> Self {
        0.0
    }
}
