use crate::{
    global::{CursorIcon, EntityId, Mat4, Quat, Vec2, Vec3, Vec4},
    internal::wit,
};
use glam::{UVec2, UVec3, UVec4};

/// Converts from a Rust representation to a wit-bindgen representation.
pub trait IntoBindgen {
    type Item;
    fn into_bindgen(self) -> Self::Item;
}

/// Converts from a wit-bindgen representation to a Rust representation.
pub trait FromBindgen {
    type Item;

    #[allow(clippy::wrong_self_convention)]
    fn from_bindgen(self) -> Self::Item;
}

impl IntoBindgen for EntityId {
    type Item = wit::types::EntityId;
    fn into_bindgen(self) -> Self::Item {
        wit::types::EntityId {
            id0: self.id0,
            id1: self.id1,
        }
    }
}
impl FromBindgen for wit::types::EntityId {
    type Item = EntityId;
    fn from_bindgen(self) -> Self::Item {
        EntityId {
            id0: self.id0,
            id1: self.id1,
        }
    }
}

impl IntoBindgen for Vec2 {
    type Item = wit::types::Vec2;
    fn into_bindgen(self) -> Self::Item {
        wit::types::Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}
impl FromBindgen for wit::types::Vec2 {
    type Item = Vec2;
    fn from_bindgen(self) -> Self::Item {
        Vec2::new(self.x, self.y)
    }
}

impl IntoBindgen for Vec3 {
    type Item = wit::types::Vec3;
    fn into_bindgen(self) -> Self::Item {
        wit::types::Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
impl FromBindgen for wit::types::Vec3 {
    type Item = Vec3;
    fn from_bindgen(self) -> Self::Item {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl IntoBindgen for Vec4 {
    type Item = wit::types::Vec4;
    fn into_bindgen(self) -> Self::Item {
        wit::types::Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}
impl FromBindgen for wit::types::Vec4 {
    type Item = Vec4;
    fn from_bindgen(self) -> Self::Item {
        Vec4::new(self.x, self.y, self.z, self.w)
    }
}

impl IntoBindgen for UVec2 {
    type Item = wit::types::Uvec2;
    fn into_bindgen(self) -> Self::Item {
        wit::types::Uvec2 {
            x: self.x,
            y: self.y,
        }
    }
}
impl FromBindgen for wit::types::Uvec2 {
    type Item = UVec2;
    fn from_bindgen(self) -> Self::Item {
        UVec2::new(self.x, self.y)
    }
}

impl IntoBindgen for UVec3 {
    type Item = wit::types::Uvec3;
    fn into_bindgen(self) -> Self::Item {
        wit::types::Uvec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
impl FromBindgen for wit::types::Uvec3 {
    type Item = UVec3;
    fn from_bindgen(self) -> Self::Item {
        UVec3::new(self.x, self.y, self.z)
    }
}

impl IntoBindgen for UVec4 {
    type Item = wit::types::Uvec4;
    fn into_bindgen(self) -> Self::Item {
        wit::types::Uvec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}
impl FromBindgen for wit::types::Uvec4 {
    type Item = UVec4;
    fn from_bindgen(self) -> Self::Item {
        UVec4::new(self.x, self.y, self.z, self.w)
    }
}

impl IntoBindgen for Quat {
    type Item = wit::types::Quat;
    fn into_bindgen(self) -> Self::Item {
        wit::types::Quat {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}
impl FromBindgen for wit::types::Quat {
    type Item = Quat;
    fn from_bindgen(self) -> Self::Item {
        Quat::from_array([self.x, self.y, self.z, self.w])
    }
}

impl IntoBindgen for Mat4 {
    type Item = wit::types::Mat4;
    fn into_bindgen(self) -> Self::Item {
        wit::types::Mat4 {
            x: self.x_axis.into_bindgen(),
            y: self.y_axis.into_bindgen(),
            z: self.z_axis.into_bindgen(),
            w: self.w_axis.into_bindgen(),
        }
    }
}
impl FromBindgen for wit::types::Mat4 {
    type Item = Mat4;
    fn from_bindgen(self) -> Self::Item {
        Mat4::from_cols(
            self.x.from_bindgen(),
            self.y.from_bindgen(),
            self.z.from_bindgen(),
            self.w.from_bindgen(),
        )
    }
}

impl IntoBindgen for CursorIcon {
    type Item = wit::client_input::CursorIcon;
    fn into_bindgen(self) -> Self::Item {
        type Ci = CursorIcon;
        type Wci = wit::client_input::CursorIcon;

        match self {
            Ci::Default => Wci::DefaultIcon,
            Ci::Crosshair => Wci::Crosshair,
            Ci::Hand => Wci::Hand,
            Ci::Arrow => Wci::Arrow,
            Ci::Move => Wci::Move,
            Ci::Text => Wci::Text,
            Ci::Wait => Wci::Wait,
            Ci::Help => Wci::Help,
            Ci::Progress => Wci::Progress,

            Ci::NotAllowed => Wci::NotAllowed,
            Ci::ContextMenu => Wci::ContextMenu,
            Ci::Cell => Wci::Cell,
            Ci::VerticalText => Wci::VerticalText,
            Ci::Alias => Wci::Alias,
            Ci::Copy => Wci::Copy,
            Ci::NoDrop => Wci::NoDrop,
            Ci::Grab => Wci::Grab,
            Ci::Grabbing => Wci::Grabbing,
            Ci::AllScroll => Wci::AllScroll,
            Ci::ZoomIn => Wci::ZoomIn,
            Ci::ZoomOut => Wci::ZoomOut,

            Ci::EResize => Wci::EResize,
            Ci::NResize => Wci::NResize,
            Ci::NeResize => Wci::NeResize,
            Ci::NwResize => Wci::NwResize,
            Ci::SResize => Wci::SResize,
            Ci::SeResize => Wci::SeResize,
            Ci::SwResize => Wci::SwResize,
            Ci::WResize => Wci::WResize,
            Ci::EwResize => Wci::EwResize,
            Ci::NsResize => Wci::NsResize,
            Ci::NeswResize => Wci::NeswResize,
            Ci::NwseResize => Wci::NwseResize,
            Ci::ColResize => Wci::ColResize,
            Ci::RowResize => Wci::RowResize,
        }
    }
}

macro_rules! bindgen_passthrough {
    ($type:ty) => {
        impl IntoBindgen for $type {
            type Item = Self;
            fn into_bindgen(self) -> Self::Item {
                self
            }
        }
        impl FromBindgen for $type {
            type Item = Self;
            fn from_bindgen(self) -> Self::Item {
                self
            }
        }
    };
}

bindgen_passthrough!(());
bindgen_passthrough!(bool);
bindgen_passthrough!(f32);
bindgen_passthrough!(f64);
bindgen_passthrough!(i32);
bindgen_passthrough!(String);
bindgen_passthrough!(u8);
bindgen_passthrough!(u32);
bindgen_passthrough!(u64);

impl<T> IntoBindgen for Option<T>
where
    T: IntoBindgen,
{
    type Item = Option<T::Item>;
    fn into_bindgen(self) -> Self::Item {
        self.map(|i| i.into_bindgen())
    }
}
impl<T> FromBindgen for Option<T>
where
    T: FromBindgen,
{
    type Item = Option<T::Item>;
    fn from_bindgen(self) -> Self::Item {
        self.map(|i| i.from_bindgen())
    }
}

impl<T> IntoBindgen for Vec<T>
where
    T: IntoBindgen,
{
    type Item = Vec<T::Item>;
    fn into_bindgen(self) -> Self::Item {
        self.into_iter().map(|i| i.into_bindgen()).collect()
    }
}
impl<T> FromBindgen for Vec<T>
where
    T: FromBindgen,
{
    type Item = Vec<T::Item>;
    fn from_bindgen(self) -> Self::Item {
        self.into_iter().map(|i| i.from_bindgen()).collect()
    }
}
