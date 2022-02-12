pub trait Lerp<'a> {
    type Write;
    fn lerp(write: Self::Write, start: &Self, end: &Self, t: f32);
}

impl<'a> Lerp<'a> for f32 {
    type Write = &'a mut Self;

    fn lerp(write: Self::Write, start: &Self, end: &Self, t: f32) {
        *write = (1.0 - t) * start + t * end
    }
}
#[cfg(feature = "glam")]
impl<'a> Lerp<'a> for glam::Mat4 {
    type Write = &'a mut Self;

    fn lerp(write: Self::Write, start: &Self, end: &Self, t: f32) {
        let (e_scale, e_rot, e_pos) = end.to_scale_rotation_translation();
        let (scale, rot, pos) = start.to_scale_rotation_translation();
        let scale = scale.lerp(e_scale, t);
        let rot = rot.slerp(e_rot, t);
        let pos = pos.lerp(e_pos, t);
        *write = glam::Mat4::from_scale_rotation_translation(scale, rot, pos);
    }
}

#[allow(unused_macros)]
macro_rules! mul_impl {
    ($ty: ident) => {
        impl<'a> Lerp<'a> for $ty {
            type Write = &'a mut Self;

            fn lerp(write: Self::Write, start: &Self, end: &Self, t: f32) {
                *write = ((1.0 - t) * *start + t * *end ) as Self
            }
        }
    };
    ([$($ty: ident),*]) => {
        $(
            mul_impl!($ty);
        )*
    };
}

#[cfg(feature = "glam")]
use glam::{Quat, Vec2, Vec3, Vec4};
#[cfg(feature = "glam")]
mul_impl!([Vec2, Vec3, Vec4]);

#[cfg(feature = "glam")]
impl<'a> Lerp<'a> for Quat {
    type Write = &'a mut Self;

    fn lerp(write: Self::Write, start: &Self, end: &Self, t: f32) {
        *write = start.slerp(*end, t)
    }
}

macro_rules! impl_tuples {
    () => {};
    ($([$idx: tt => $name: ident]), *) => {
        impl<'a, $($name: Lerp<'a> + 'a),*> Lerp<'a> for ($($name,)*) {
        type Write = ($(<$name as Lerp<'a>>::Write,)*);
            fn lerp(write: Self::Write, start: &Self,end: &Self, t: f32) {
                $(
                 $name::lerp(write.$idx, &start.$idx, &end.$idx, t);
                ) *
            }
        }
    };
}

impl_tuples!([0 => A]);
impl_tuples!([0 => A], [1 => B]);
impl_tuples!([0 => A], [1 => B], [2 => C]);
impl_tuples!([0 => A], [1 => B], [2 => C], [3 => D]);
impl_tuples!([0 => A], [1 => B], [2 => C], [3 => D], [4 => E], [5 => F]);
impl_tuples!([0 => A], [1 => B], [2 => C], [3 => D], [4 => E], [5 => F], [6 => G]);
impl_tuples!([0 => A], [1 => B], [2 => C], [3 => D], [4 => E], [5 => F], [6 => G], [7 => H]);
