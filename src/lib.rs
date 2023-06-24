#![recursion_limit = "256"]
#![deny(unreachable_pub)]
#![deny(unused_extern_crates)]
#![deny(unused_allocation)]
#![deny(unused_assignments)]
#![deny(unused_comparisons)]
#![deny(warnings)]
// #![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::broken_intra_doc_links))]
#![allow(clippy::approx_constant)]
#![allow(clippy::float_cmp)]
#![allow(clippy::match_wild_err_arm)]
#![allow(clippy::new_ret_no_self)]
#![allow(clippy::type_complexity)]
#![allow(clippy::unit_arg)]
#![deny(clippy::clone_on_ref_ptr)]
#![deny(clippy::trivially_copy_pass_by_ref)]
#![deny(clippy::disallowed_methods)]
#![deny(clippy::missing_const_for_fn)]


/// The current version of LogShark in simplified format.
/// `<version-number>-nightly`.
pub fn logshark_version() -> impl std::fmt::Display {
    #[cfg(feature = "nightly")]
        let pkg_version = format!("{}-nightly", built_info::PKG_VERSION);

    #[cfg(not(feature = "nightly"))]
        let pkg_version = match built_info::DEBUG {
        // If any debug info is included, consider it a non-release build.
        "1" | "2" | "true" => {
            format!(
                "{}-custom-{}",
                built_info::PKG_VERSION,
                built_info::GIT_SHORT_HASH
            )
        }
        _ => built_info::PKG_VERSION.to_string(),
    };

    pkg_version
}

/// Returns a string containing full version information of the current build.
pub fn get_version() -> String {
    let pkg_version = logshark_version();
    let build_desc = built_info::VECTOR_BUILD_DESC;
    let build_string =  match build_desc {
        Some(desc) => format!("{} {}", built_info::TARGET, desc),
        None => built_info::TARGET.into(),
    };

    let build_string = match built_info::DEBUG {
        "1" => format!("{} debug=line", build_string),
        "2" | "true" => format!("{} debug=full", build_string),
        _ => build_string,
    };

    format!("{} ({})", pkg_version, build_string)

}

/// Includes information about the current build.
#[allow(warnings)]
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
