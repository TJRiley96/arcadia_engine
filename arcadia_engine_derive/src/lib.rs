use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote;


fn impl_component_trait(ast: &DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let ident_string = ident.to_string();
    quote::quote! {
        impl ComponentTrait for #ident {
            fn update(&mut self, delta_time: f32) {
                // This is a placeholder method for updating the component each frame.
                // The actual implementation will depend on the specific behavior of the component.
            }

            fn process_input(&mut self, input: u8) {
                // This is a placeholder method for processing input for the component.
                // The actual implementation will depend on the specific behavior of the component and how it interacts with input.
            }

            fn on_update_world_tranform(&mut self) {
                // This is a placeholder method for handling updates to the world transform of the component.
                // The actual implementation will depend on the specific behavior of the component and how it interacts with the world transform.
            }

            fn get_component_name(&self) -> &'static str {
                #ident_string
            }

        }
    }.into()
}

#[proc_macro_derive(ComponentTrait)]
pub fn component_trait_derive(input: TokenStream) -> TokenStream {
    // This is a placeholder implementation for the ComponentTrait derive macro.
    // The actual implementation will depend on the specific behavior of the ComponentTrait and how we want to generate code for it.
    let ast: DeriveInput = syn::parse(input).unwrap();

    impl_component_trait(&ast)

}

