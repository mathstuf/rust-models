use crate::prelude::*;

pub struct Getter {
    table_name: Ident,
    getter_name: Ident,
}

impl ToTokens for Getter {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Getter {
            table_name,
            getter_name,
        } = self;

        tokens.extend(quote! {
            impl #table_name {
                async fn #getter_name () {
                }
            }
        })
    }
}
