#![allow(unused_macros)]

// Public macro: generate a module containing
// - Active: which pattern is active
// - SetParam: typed payload for updating params of a specific inner pattern
// - Params: { Set(SetParam), Toggle, Select(Active) }
// - Iter: enum over all inner iterator types
// - Switch: the Pattern wrapper implementing Pattern for the provided types
//
// Usage (in a downstream crate):
// blinksy::pattern_switch! {
//     pub mod StripPatterns {
//         Rainbow: blinksy::patterns::rainbow::Rainbow,
//         Noise: blinksy::patterns::noise::Noise1d<
//             blinksy::patterns::noise::noise_fns::Perlin
//         >,
//     }
// }
//
// Then build a Control with `StripPatterns::Switch`.
//
// Notes:
// - Each inner Pattern's Params must implement Default.
// - All inner Patterns must share the same Color type (enforced).
// - Toggle cycles in the order listed; last wraps to first.

#[macro_export]
macro_rules! pattern_switch {
    (
        $(#[$meta:meta])*
        $vis:vis mod $mod_name:ident {
            $first_name:ident : $first_type:ty
            $(, $rest_name:ident : $rest_type:ty )* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis mod $mod_name {
            use $crate::pattern::Pattern as PatternTrait;

            #[derive(Copy, Clone, Debug, Eq, PartialEq)]
            pub enum Active {
                $first_name,
                $( $rest_name, )*
            }

            pub enum Iter<I_$first_name $(, I_$rest_name)*> {
                $first_name(I_$first_name),
                $( $rest_name(I_$rest_name), )*
            }

            impl<T, I_$first_name $(, I_$rest_name)*>
                core::iter::Iterator for Iter<I_$first_name $(, I_$rest_name)*>
            where
                I_$first_name: Iterator<Item = T>,
                $( I_$rest_name: Iterator<Item = T>, )*
            {
                type Item = T;
                fn next(&mut self) -> Option<Self::Item> {
                    match self {
                        Self::$first_name(i) => i.next(),
                        $( Self::$rest_name(i) => i.next(), )*
                    }
                }
            }

            pub enum SetParam<P_$first_name $(, P_$rest_name)*> {
                $first_name(P_$first_name),
                $( $rest_name(P_$rest_name), )*
            }

            pub enum Params<P_$first_name $(, P_$rest_name)*> {
                Set(SetParam<P_$first_name $(, P_$rest_name)*>),
                Toggle,
                Select(Active),
            }

            pub struct Switch {
                pub $first_name: $first_type,
                $( pub $rest_name: $rest_type, )*
                pub active: Active,
            }

            impl<Dim, Layout> PatternTrait<Dim, Layout> for Switch
            where
                $first_type: PatternTrait<Dim, Layout>,
                $(
                    $rest_type: PatternTrait<
                        Dim,
                        Layout,
                        Color = <$first_type as PatternTrait<Dim, Layout>>::Color
                    >,
                )*
                // Require Default params for all inner patterns
                <$first_type as PatternTrait<Dim, Layout>>::Params: Default,
                $( <$rest_type as PatternTrait<Dim, Layout>>::Params: Default, )*
            {
                type Color = <$first_type as PatternTrait<Dim, Layout>>::Color;

                type Params = Params<
                    <$first_type as PatternTrait<Dim, Layout>>::Params
                    $( , <$rest_type as PatternTrait<Dim, Layout>>::Params )*
                >;

                type Iter = Iter<
                    <$first_type as PatternTrait<Dim, Layout>>::Iter
                    $( , <$rest_type as PatternTrait<Dim, Layout>>::Iter )*
                >;

                fn new(params: Self::Params) -> Self {
                    let mut s = Self {
                        $first_name: <$first_type as PatternTrait<
                            Dim,
                            Layout
                        >>::new(Default::default()),
                        $(
                            $rest_name: <$rest_type as PatternTrait<
                                Dim,
                                Layout
                            >>::new(Default::default()),
                        )*
                        active: Active::$first_name,
                    };
                    s.set(params);
                    s
                }

                fn set(&mut self, params: Self::Params) {
                    match params {
                        Params::Set(sp) => match sp {
                            SetParam::$first_name(p) => self.$first_name.set(p),
                            $( SetParam::$rest_name(p) => self.$rest_name.set(p), )*
                        },
                        Params::Toggle => {
                            self.active = match self.active {
                                $crate::cycle_arms!(
                                    Active;
                                    $first_name;
                                    $first_name $(, $rest_name)*
                                )
                            };
                        }
                        Params::Select(a) => {
                            self.active = a;
                        }
                    }
                }

                fn tick(&mut self, time_in_ms: u64) -> Self::Iter {
                    match self.active {
                        Active::$first_name => {
                            Iter::$first_name(self.$first_name.tick(time_in_ms))
                        }
                        $(
                            Active::$rest_name => {
                                Iter::$rest_name(self.$rest_name.tick(time_in_ms))
                            }
                        )*
                    }
                }
            }
        }
    };
}

// Helper: generate match arms like A => B, B => C, ..., Last => First
#[macro_export]
macro_rules! cycle_arms {
    ($name:ident; $first:ident; $current:ident, $next:ident $(, $rest:ident)*) => {
        $name::$current => $name::$next,
        $crate::cycle_arms!($name; $first; $next $(, $rest)*)
    };
    ($name:ident; $first:ident; $last:ident) => {
        $name::$last => $name::$first,
    };
}
