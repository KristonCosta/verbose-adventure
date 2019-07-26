#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(VertexAttribPointers, attributes(location))]
pub fn vertex_attrib_pointers_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    let generated = generate_impl(&ast);
    TokenStream::from(generated)
}

fn generate_impl(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;
    let fields_vertex_attrib_pointer = generate_vertex_attrib_pointer_calls(&ast.data);

    quote!(
        impl #ident #generics #where_clause {
            pub fn vertex_attrib_pointers(gl: &::gl::Gl) {
                let stride = std::mem::size_of::<Self>();
                let offset = 0;
                #(#fields_vertex_attrib_pointer)*
            }
        }
    )
}

fn generate_vertex_attrib_pointer_calls(body: &syn::Data) -> Vec<proc_macro2::TokenStream> {
    match body {
        &syn::Data::Struct(syn::DataStruct {fields: ref s, ..}) =>
            s.iter().map(generate_struct_field_vertex_attrib_pointer_call).collect(),
        &syn::Data::Enum(_) => panic!("VertexAttribPointers cannot be implemented for enums."),
        &syn::Data::Union(_) => panic!("VertexAttribPointers cannot be implemented for unions."),
    }
}

fn generate_struct_field_vertex_attrib_pointer_call(field: &syn::Field) -> proc_macro2::TokenStream {
    let field_name = match field.ident {
        Some(ref i) => format!("{}", i),
        None => String::from(""),
    };

    let location_attr = field.attrs
        .iter()
        .filter(|a| a.path.is_ident("location"))
        .next()
        .unwrap_or_else(|| panic!(
            "Field {} is missing #[location = ?] attribute", &field_name
            ));


    let location_value: usize = location_attr
        .parse_meta()
        .map(|meta| match meta {
            syn::Meta::NameValue(ref name_value) => {
                match &name_value.lit {
                    syn::Lit::Int(int) => int.value() as usize,
                    _ => panic!("Field {} location attribute value must contain an integer", &field_name),
                }
            }
            _ => panic!("Field {} location should have the structure #[location = ?]")
        })
        .unwrap_or_else(|_| panic!(
        "Field {} location attribute missing a value", field_name)
    );

    let field_type = &field.ty;
    quote! {
        let location = #location_value;
        unsafe {
            #field_type::vertex_attrib_pointer(gl, stride, location, offset);
        }
        let offset = offset + ::std::mem::size_of::<#field_type>();
    }
}