/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       notes
 * 2025-10-10     foxglove     latest version
 */
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use syn::{parse, spanned::Spanned, ReturnType, Visibility, Meta};
use syn::parse::Parser;
use syn::NestedMeta as DarlingNestedMeta;

#[derive(Debug, FromMeta)]
struct Args {
    #[darling(default)]
    appname: Option<String>,
    #[darling(default)]
    run: Option<bool>,
    #[darling(default)]
    cmd: Option<bool>,
    #[darling(default)]
    desc: Option<String>,
}

#[proc_macro_attribute]
pub fn rtt_main(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as syn::ItemFn);
    let parser = syn::punctuated::Punctuated::<Meta, syn::token::Comma>::parse_terminated;
    let raw_meta = match parser.parse(args) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };
    let raw_arg: Vec<DarlingNestedMeta> = raw_meta
        .into_iter()
        .map(DarlingNestedMeta::Meta)
        .collect();
    let parg = Args::from_list(&raw_arg).map_err(|e| e.write_errors());
    let arg = match parg {
        Ok(x) => x,
        Err(e) => {
            return e.into();
        }
    };

    if arg.appname.is_none() {
        return parse::Error::new(
            Span::call_site(),
            "`#[marco_main_use]` macro must have attribute `appname`",
        )
            .to_compile_error()
            .into();
    }

    let main_func_name = format_ident!("__{}_main_func", arg.appname.as_ref().unwrap());
    // let main_func_name = format_ident!("main");

    let run_seg_name = format_ident!("__{}_run_seg", arg.appname.as_ref().unwrap());
    let run_func_name = format_ident!("__{}_run_func", arg.appname.as_ref().unwrap());
    let run_struct_name = format_ident!("__{}_run_seg_struct", arg.appname.as_ref().unwrap());
    let cmd_seg_name = format_ident!("__{}_cmd_seg", arg.appname.as_ref().unwrap());
    let cmd_struct_name = format_ident!("__{}_cmd_seg_struct", arg.appname.as_ref().unwrap());
    let cmd_namestr_name = format_ident!("__{}_cmd_namestr", arg.appname.as_ref().unwrap());
    let cmd_descstr_name = format_ident!("__{}_cmd_descstr", arg.appname.as_ref().unwrap());

    let mod_name = format_ident!("__app_init_{}_", arg.appname.as_ref().unwrap());
    let call_func_name = f.sig.ident.clone();

    // check the function signature
    let valid_signature = f.sig.constness.is_none()
        && f.sig.unsafety.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.inputs.len() == 1
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
        ReturnType::Default => true,
        _ => false,
    };

    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[entry]` function must have signature `fn(arg: vec::IntoIter<&[u8]>)`",
        )
            .to_compile_error()
            .into();
    }

    let attrs = f.attrs.clone();

    let origin = quote!(
        #(#attrs)*
        #f
    );

    let core = quote!(
        #[no_mangle]
        pub unsafe extern "C" fn #main_func_name(argc: u32, argv: *const *const u8) {
            use core::iter::Iterator;
            use rt_rust::param::ParamItem;
            let vec = {
                (0..argc as isize)
                    .map(|i| {
                        let mut len = 0usize;
                        loop {
                            if *(*argv.offset(i)).offset(len as isize) != b'\0' {
                                len += 1;
                            } else {
                                break
                            }
                        }
                        ParamItem::new(core::slice::from_raw_parts::<'static, _>(*argv.offset(i), len))
                    })
                    .collect::<Vec<_>>()
            };
            #call_func_name (vec.into_iter())
        }
    );

    let run_seg = if arg.run.is_none() {
        quote!()
    } else {
        // Place the init function pointer directly in .rti_fn.6 as expected by RT-Thread
        quote!(
            #[no_mangle]
            pub extern "C" fn #run_func_name() -> i32 {
                unsafe { #main_func_name(0, core::ptr::null()) };
                0
            }

            // RT-Thread expects an init_fn_t (int (*)(void)) element in .rti_fn.6
            #[link_section = ".rti_fn.6"]
            #[no_mangle]
            static #run_seg_name: extern "C" fn() -> i32 = #run_func_name;
        )
    };

    let cmd_seg = if arg.cmd.is_none() {
        quote!()
    } else {
        let desc = arg.desc.map_or(String::from("No desc\0"), |mut x| {
            x.push_str("\0");
            x
        });
        let r_desc = Literal::byte_string(desc.as_bytes());

        let mut cmd_name = arg.appname.as_ref().unwrap().clone();
        cmd_name.push_str("\0");
        let r_cmd_name = Literal::byte_string(cmd_name.as_bytes());

        let desc_len = desc.len();
        let cmd_name_len = cmd_name.len();
        quote!(
            #[link_section = ".rodata.name"]
            #[no_mangle]
            static #cmd_namestr_name: [u8; #cmd_name_len] = *#r_cmd_name;
            #[link_section = ".rodata.name"]
            #[no_mangle]
            static #cmd_descstr_name: [u8; #desc_len] = *#r_desc;

            // Finsh syscall layout when FINSH_USING_DESCRIPTION and FINSH_USING_OPTION_COMPLETION enabled:
            // struct finsh_syscall { const char *name; const char *desc; const struct msh_cmd_opt *opt; syscall_func func; };
            // We don't provide options from Rust, set opt to null.
            #[repr(C)]
            struct #cmd_struct_name {
                name: *const u8,
                desc: *const u8,
                opt: *const core::ffi::c_void,
                func: extern "C" fn(argc: u32, argv: *const *const u8) -> i32,
            }
            unsafe impl Sync for #cmd_struct_name{}

            extern "C" fn __wrap_main(argc: u32, argv: *const *const u8) -> i32 {
                // Call the generated main and return 0 consistently
                unsafe { #main_func_name(argc, argv); }
                0
            }

            #[link_section = "FSymTab"]
            #[no_mangle]
            static #cmd_seg_name: #cmd_struct_name = #cmd_struct_name {
                name: #cmd_namestr_name.as_ptr(),
                desc: #cmd_descstr_name.as_ptr(),
                opt: core::ptr::null(),
                func: __wrap_main,
            };
        )
    };    

    quote!(
        #origin
        mod #mod_name {
            use super::#call_func_name;
            use core::marker::Sync;
            extern crate alloc;
            use alloc::vec::Vec;
            use core::iter::IntoIterator;

            #core
            #run_seg
            #cmd_seg
        }
    )
        .into()
}