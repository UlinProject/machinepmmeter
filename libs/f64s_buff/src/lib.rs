// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Denis Kotlyarov (Денис Котляров) <denis2005991@gmail.com>

#[cfg(any(test, not(feature = "f64_string_optimized")))]
mod default;
#[cfg(any(test, feature = "f64_string_optimized"))]
mod optimized;

#[cfg(not(feature = "f64_string_optimized"))]
pub use crate::default::F64SBuff;
#[cfg(not(feature = "f64_string_optimized"))]
pub use crate::default::F64SBuffAutoClear;

#[cfg(feature = "f64_string_optimized")]
pub use crate::optimized::F64SBuff;

#[cfg(feature = "f64_string_optimized")]
pub use crate::optimized::F64SBuffAutoClear;
