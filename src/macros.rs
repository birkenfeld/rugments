// Copyright (c) 2006-2015 by the respective authors (see AUTHORS file).
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
// * Redistributions of source code must retain the above copyright
//   notice, this list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright
//   notice, this list of conditions and the following disclaimer in the
//   documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use lexer::{MatchAction, StateAction};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleDef {
    Regex(&'static str, MatchAction, StateAction),
    Words(&'static [&'static str], MatchAction, StateAction),
    // Inherit(...),
    Default(StateAction),
}

pub type MachineDef = &'static [(&'static str, &'static [RuleDef])];

macro_rules! machine_as_type {
    (@expr $x:expr) => { $x };
    (@slice ($($x:expr),*)) => { &[$($x),*] };
    (@ttslice ($($x:ident),*)) => { &[$($crate::token::$x),*] };
}

macro_rules! machine_state_action {
    ("#pop") => ($crate::lexer::StateAction::Pop);
    (($($st:tt)*)) => ($crate::lexer::StateAction::PushMulti(machine_as_type!(@slice ($($st)*))));
    ($expr:expr) => ($crate::lexer::StateAction::Push($expr));
}

macro_rules! machine_action {
    ($rx:expr, bygroups $tts:tt) => {
        $crate::macros::RuleDef::Regex($rx, $crate::lexer::MatchAction::ByGroups(
            machine_as_type!(@ttslice $tts)), $crate::lexer::StateAction::None) };
    ($rx:expr, bygroups $tts:tt, $($sa:tt)*) => {
        $crate::macros::RuleDef::Regex($rx, $crate::lexer::MatchAction::ByGroups(
            machine_as_type!(@ttslice $tts)), machine_state_action!($($sa)*)) };
    ($rx:expr, $tt:ident) => {
        $crate::macros::RuleDef::Regex($rx, $crate::lexer::MatchAction::Single(
            machine_as_type!(@expr $crate::token::$tt)), $crate::lexer::StateAction::None) };
    ($rx:expr, $tt:ident, $($sa:tt)*) => {
        $crate::macros::RuleDef::Regex($rx, $crate::lexer::MatchAction::Single(
            machine_as_type!(@expr $crate::token::$tt)), machine_state_action!($($sa)*)) };
}

macro_rules! machine_regex_prefix {
    ()  => { r"\A" };
    (,) => { r"\A" };
    (IGNORECASE, $($opts:tt)*) => { concat!("(?i)", machine_regex_prefix!($($opts),*)) };
    (DOTALL,     $($opts:tt)*) => { concat!("(?s)", machine_regex_prefix!($($opts),*)) };
}

macro_rules! machine_rule {
    ([$($opt:tt)*], ($rx:expr, $($action:tt)*)) => {
        machine_action!(concat!(machine_regex_prefix!($($opt)*,), $rx), $($action)*)
    };
}

#[macro_export]
macro_rules! define_machine {
    ($name:ident, $raw_name:ident, $opt:tt,
     $($state:tt : [$($statett:tt),* $(,)*]),* $(,)*) =>
    {
        const $raw_name: $crate::macros::MachineDef = &[$(
            (machine_as_type!(@expr $state), &[$(
                machine_rule!($opt, $statett)
            ),*])
        ),*];
        lazy_static! {
            pub static ref $name: $crate::lexer::Machine =
                $crate::lexer::Machine::convert($raw_name);
        }
    };
}

#[macro_export]
macro_rules! define_lexer {
    ($name:ident,
     $machine_name:ident,
    ) => {
        pub struct $name<'t> {
            inner: $crate::lexer::RegexLexer<'t>
        }

        impl<'t> $name<'t> {
            pub fn new(text: &'t str) -> $name<'t> {
                $name {
                    inner: $crate::lexer::RegexLexer::new(
                        &$machine_name, "root", text)
                }
            }
        }

        impl<'t> Iterator for $name<'t> {
            type Item = $crate::token::Token<'t>;

            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next()
            }
        }

        impl<'t> $crate::lexer::Lexer<'t> for $name<'t> { }
    }
}


#[macro_export]
macro_rules! add_option {
    ($name:ident, $setter:ident, $ty:ident) => {
        pub fn $setter(mut self, value: $ty) -> Self {
            self.$name = value;
            self
        }
    }
}
