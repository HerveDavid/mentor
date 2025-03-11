use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type};

pub fn impl_identifiable_trait(ast: DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let register_impl = generate_register_impl(&ast.data);

    let expanded = quote! {
        impl #impl_generics crate::extensions::Identifiable for #name #ty_generics #where_clause {
            fn id(&self) -> String {
                self.id.clone()
            }

            fn register(&self, world: &mut bevy_ecs::world::World, schedule: &mut bevy_ecs::schedule::Schedule) {
                {
                    let mut event_writer = world.resource_mut::<bevy_ecs::event::Events<crate::plugins::RegisterEvent>>();
                    event_writer.send(crate::plugins::RegisterEvent {
                        component: crate::identifiable::Identifiables::#name(self.clone()),
                    });
                }

                #register_impl

                schedule.run(world);
            }

            fn register_system(&self, mut commands: &mut bevy_ecs::system::Commands, registery: &mut bevy_ecs::system::ResMut<crate::resources::AssetRegistry>) {
                registery.add_component(&mut commands, self.id(), self.clone());
            }
        }
    };

    expanded
}

// Générer du code pour enregistrer les champs qui semblent être identifiables
fn generate_register_impl(data: &Data) -> TokenStream {
    match data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields) => {
                    // On ne génère du code que pour les champs qui sont probablement Identifiable
                    let field_registers = fields.named.iter().filter_map(|field| {
                        let field_name = &field.ident;
                        let field_type = &field.ty;

                        // Ignorer les types primitifs et les enums
                        if is_primitive_or_enum(field_type) {
                            return None;
                        }

                        // Traiter les vecteurs et les types normaux
                        if is_vec_type(field_type) {
                            Some(quote! {
                                // Pour les vecteurs, on tente d'enregistrer chaque élément
                                for item in &self.#field_name {
                                    // On va supposer que tout élément d'un vecteur qui n'est pas
                                    // considéré comme primitif ou enum pourrait implémenter Identifiable
                                    item.register(world, schedule);
                                }
                            })
                        } else {
                            // Pour les champs ordinaires
                            Some(quote! {
                                // On suppose que le champ implémente Identifiable
                                self.#field_name.register(world, schedule);
                            })
                        }
                    });

                    quote! {
                        #(#field_registers)*
                    }
                }
                _ => quote! {},
            }
        }
        _ => quote! {},
    }
}

// Vérifier si un type est un Vec<T>
fn is_vec_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Vec";
        }
    }
    false
}

// Vérifier si un type est primitif ou un enum
fn is_primitive_or_enum(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let type_name = segment.ident.to_string();

            // Vérifier d'abord si c'est un Vec pour examiner son contenu
            if type_name == "Vec" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        return is_primitive_or_enum(inner_type);
                    }
                }
                return false;
            }

            // Types primitifs et conteneurs usuels
            let primitives = [
                "String", "i32", "i64", "f32", "f64", "bool", "char", "u8", "u16", "u32", "u64",
                "usize", "isize", "Vec", "Option", "DateTime", "Property",
            ];

            if primitives.contains(&type_name.as_str()) {
                return true;
            }

            // Types qui sont probablement des enums (convention de nommage)
            return type_name.ends_with("Kind")
                || type_name.ends_with("Type")
                || type_name.ends_with("Mode")
                || type_name == "Side"
                || type_name == "EnergySource";
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_is_vec_type() {
        let vec_type: Type = parse_quote!(Vec<String>);
        assert!(
            is_vec_type(&vec_type),
            "Vec<String> should be detected as Vec"
        );

        let non_vec_type: Type = parse_quote!(String);
        assert!(
            !is_vec_type(&non_vec_type),
            "String should not be detected as Vec"
        );
    }

    #[test]
    fn test_is_primitive_or_enum() {
        // Primitives
        let string_type: Type = parse_quote!(String);
        assert!(
            is_primitive_or_enum(&string_type),
            "String should be detected as primitive"
        );

        // Enums (by naming convention)
        let enum_type: Type = parse_quote!(TopologyKind);
        assert!(
            is_primitive_or_enum(&enum_type),
            "TopologyKind should be detected as enum"
        );

        // Regular struct types
        let struct_type: Type = parse_quote!(Network);
        assert!(
            !is_primitive_or_enum(&struct_type),
            "Network should not be detected as primitive or enum"
        );

        // Vector of enums
        let vec_enum_type: Type = parse_quote!(Vec<EnergySource>);
        assert!(
            is_primitive_or_enum(&vec_enum_type),
            "Vec<EnergySource> should be detected as containing enum"
        );

        // Vector of structs
        let vec_struct_type: Type = parse_quote!(Vec<Network>);
        assert!(
            !is_primitive_or_enum(&vec_struct_type),
            "Vec<Network> should not be detected as primitive or enum"
        );
    }
}
