/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use quote::quote;
use std::path::{Path, PathBuf};

use crate::api_parser::*;
use crate::class_generator::make_utility_function_definition;
use crate::write_formatted::write_formatted_if_needed;
use crate::Context;

pub(crate) fn generate_utilities_file(
    api: &ExtensionApi,
    ctx: &mut Context,
    gen_path: &Path,
    out_files: &mut Vec<PathBuf>,
) {
    let mut utility_fn_defs = vec![];
    for utility_fn in &api.utility_functions {
        // note: category unused -> could be their own mod

        let def = make_utility_function_definition(utility_fn, ctx);
        utility_fn_defs.push(def);
    }

    let tokens = quote! {
        use godot_ffi as sys;
        use crate::builtin::*;
        use crate::obj::Gd;
        use crate::engine::Object;
        use sys::GodotFfi as _;

        #(#utility_fn_defs)*
    };

    write_formatted_if_needed(tokens, gen_path, "utilities.rs", out_files);
}
