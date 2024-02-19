extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Instruction)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
  let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

  let variants = if let syn::Data::Enum(e) = data {
    e.variants.into_iter().collect::<Vec<_>>()
  } else {
    panic!("Can only derive Instruction on enums.");
  };

  let use_variants = variants.iter().map(|v| {
    let ident_lower = ident.to_string().to_lowercase();
    let variant_ident = &v.ident;
    let variant_ident_lower = variant_ident.to_string().to_lowercase();
    let use_stmt: proc_macro2::TokenStream =
      format!("use crate::core::{ident_lower}::{variant_ident_lower}::{variant_ident};")
        .parse()
        .expect("failed to parse use statement");
    let mod_stmt: proc_macro2::TokenStream = format!("pub mod {variant_ident_lower};")
      .parse()
      .expect("failed to parse mod statement");
    quote! {
      #use_stmt
      #mod_stmt
    }
  });

  let decode_variants = variants.iter().map(|v| {
    let variant_ident = &v.ident;
    quote! {
      opcode if #variant_ident::opcodes().contains(&opcode) => #variant_ident::decode(cpu, opcode),
    }
  });

  let execute_variants = variants.iter().map(|v| {
    let variant_ident = &v.ident;
    quote! {
      #ident::#variant_ident(i) => i.execute(cpu),
    }
  });

  let expanded = quote! {
    #( #use_variants )*

    impl #ident {
      pub fn decode(cpu: &mut Cpu, opcode: u8) -> Result<#ident, CpuError> {
        match opcode {
          #( #decode_variants )*
          _ => Err(CpuError::InvalidOpCode(opcode)),
        }
      }

      pub fn execute(&self, cpu: &mut Cpu) -> Result<(), CpuError> {
        match self {
          #( #execute_variants )*
        }
      }
    }
  };

  // Hand the output tokens back to the compiler
  expanded.into()
}
