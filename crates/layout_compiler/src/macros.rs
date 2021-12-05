#![allow(unused)]

macro_rules! define_enum_macro {
    ($Type:ident, $($variant:ident),+ $(,)?) => {
        define_enum_macro!{#internal, [$], $Type, $($variant),+}
    };
    (#internal, [$dollar:tt], $Type:ident, $($variant:ident),+) => {
        macro_rules! $Type {
            ($dollar($field:ident $dollar(: $p:pat)?,)* ..) => {
                $($Type::$variant { $dollar($field $dollar(: $p)?,)* .. } )|+
            }
        }
    };
}

pub(crate) use define_enum_macro;
