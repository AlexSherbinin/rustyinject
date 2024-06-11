#![deny(
    warnings,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::restriction,
    clippy::cargo
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::blanket_clippy_restriction_lints,
    clippy::missing_inline_in_public_items,
    clippy::single_char_lifetime_names,
    clippy::implicit_return,
    clippy::pattern_type_mismatch,
    clippy::question_mark_used,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::pub_with_shorthand,
    clippy::absolute_paths,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::multiple_crate_versions,
    clippy::missing_docs_in_private_items,
    clippy::pub_use,
    clippy::infinite_loop, // Allowed because of bug: https://github.com/rust-lang/rust-clippy/issues/12338
    clippy::unseparated_literal_suffix,
    clippy::self_named_module_files,
    clippy::big_endian_bytes,
    clippy::single_call_fn,
    clippy::missing_trait_methods,
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    clippy::print_stdout,
    clippy::shadow_unrelated,
    clippy::undocumented_unsafe_blocks,
    clippy::as_conversions,
    clippy::ref_as_ptr,
    clippy::doc_markdown,
    clippy::unwrap_used,
    clippy::unreachable,
    clippy::impl_trait_in_params,
)]
#![forbid(unreachable_pub)]

extern crate alloc;

mod container;
mod deps_list;
pub mod injector;

pub mod indecies {
    //! Indecies for indexing [`DepsList`](super::deps_list::DepsList) and
    //! [`DependencyContainer`](super::container::DependencyContainer)
    pub use super::container::{CurrentScope, ParentScope};
    pub use super::deps_list::{Last, Next};
}
pub use container::DependencyContainer;
pub use deps_list::{DepsList, DepsListGetMut, DepsListGetRef, DepsListRemove};
