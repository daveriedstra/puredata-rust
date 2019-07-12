#![recursion_limit = "256"]

extern crate proc_macro;

use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Expr, ExprBlock, GenericParam, Generics, Ident, ItemImpl,
    LitStr, Token, Type, Visibility,
};

struct ProcessorExternal {
    struct_name: Ident,
    struct_contents: ExprBlock,
    impl_blocks: Vec<ItemImpl>,
}

impl Parse for ProcessorExternal {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![pub]>()?;
        input.parse::<Token![struct]>()?;
        let struct_name: Ident = input.parse()?;
        let struct_contents: ExprBlock = input.parse()?;

        let mut impl_blocks = Vec::new();
        let iblock: ItemImpl = input.parse()?;
        impl_blocks.push(iblock);

        Ok(ProcessorExternal {
            struct_name,
            struct_contents,
            impl_blocks,
        })
    }
}

#[proc_macro]
pub fn external_processor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ProcessorExternal {
        struct_name,
        struct_contents,
        impl_blocks,
    } = parse_macro_input!(input as ProcessorExternal);

    let pdname = struct_name.to_string().to_lowercase();
    let flat_name = pdname.clone() + "_tilde";

    let class_name = LitStr::new(&(pdname.clone() + "~"), Span::call_site());
    let class_object = Ident::new(&(pdname.to_uppercase() + "_CLASS"), Span::call_site());
    let new_method = Ident::new(&(flat_name.clone() + "_new"), Span::call_site());
    let free_method = Ident::new(&(flat_name.clone() + "_free"), Span::call_site());
    let dsp_method = Ident::new(&(flat_name.clone() + "_dsp"), Span::call_site());
    let perform_method = Ident::new(&(flat_name.clone() + "_perform"), Span::call_site());
    let setup_method = Ident::new(&(flat_name.clone() + "_setup"), Span::call_site());

    let iblock = &impl_blocks[0]; //allow for more than 1

    let expanded = quote! {
        //original struct
        pub struct #struct_name #struct_contents

        //generated
        type Wrapped = SignalProcessorExternalWrapper<#struct_name>;
        static mut #class_object: Option<*mut puredata_sys::_class> = None;

        //new trampoline
        pub unsafe extern "C" fn #new_method () -> *mut ::std::os::raw::c_void {
            Wrapped::new(#class_object.expect("hello dsp class not set"))
        }

        //free trampoline
        pub unsafe extern "C" fn #free_method (x: *mut Wrapped) {
            let x = &mut *x;
            x.free();
        }

        pub unsafe extern "C" fn #dsp_method(
            x: *mut Wrapped,
            sp: *mut *mut puredata_sys::t_signal,
            ) {
            let x = &mut *x;
            x.dsp(sp, #perform_method);
        }

        pub unsafe extern "C" fn #perform_method(
            w: *mut puredata_sys::t_int,
            ) -> *mut puredata_sys::t_int {
            //actually longer than 2 but .offset(1) didn't seem to work correctly
            //but slice does
            let x = std::slice::from_raw_parts(w, 2);
            let x = &mut *std::mem::transmute::<_, *mut Wrapped>(x[1]);
            x.perform(w)
        }

        #[no_mangle]
        pub unsafe extern "C" fn #setup_method() {
            let name = CString::new(#class_name).expect("CString::new failed");
            let mut c = Class::<Wrapped>::register_dsp_new(
                name,
                #new_method,
                SignalClassType::WithInput(
                    #dsp_method,
                    Wrapped::float_convert_field_offset(),
                ),
                Some(#free_method),
            );
            //c.add_method(Method::Bang(hellodsp_tilde_bang_trampoline));

            //let name = CString::new("blah").expect("CString::new failed");
            //c.add_method(Method::SelF1(name, hellodsp_tilde_float_trampoline, 1));

            #class_object = Some(c.into());
        }

        #iblock //XXX actually mutate
    };
    proc_macro::TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
