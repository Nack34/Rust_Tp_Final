#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod Rust_Tp_Final {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    // no es mejor usar #[ink::storage_item] ¿? segun el ink implementa todos los traits
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Fecha{ // se debe crear con lo que nos conto el profe
        año:u32,
        mes:u8,
        dia:u16
    }
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum CategoriasDisponibles{
        A,
        B{id_deporte:u32},
        C
    }
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Actividades{
        Futbol,
        Gimnasio,
        Basquet,
        Rugby,
        Hockey,
        Natacion,
        Tenis,
        Paddle
    }
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct DatosPersonalesSocio{
        nombre:String,
        apellidos:String
    }
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Categoria{
        id:u32,
        nombre:String,
        costo_mensual_en_tokens:u32,
        //id_de_actividades_accesibles: no encuentro un hashset
    }
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Socio{
        id:u32,
        categoria_id:u32,
        datos_personales:DatosPersonalesSocio
    }
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Pago{
        id:u32,
        fecha_de_pago:Option<Fecha>,
        fecha_de_vencimiento:Fecha,
        monto:u128,//   no deja utilizar f32 o f64 y no se por que?
        pago_con_bonificacion:bool
    }
    #[ink(storage)]
    pub struct ClubSemRust {
        usuarios:Mapping<u32,Socio>,
        actividades:Mapping<u32,Actividades>,
        categorias:Mapping<u32,Categoria>,
        cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento:u32,
        pagos:Vec<Pagos>
    }

    impl ClubSemRust {
    }


    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut ClubSemRust = ClubSemRust::new(false);
            assert_eq!(ClubSemRust.get(), false);
            ClubSemRust.flip();
            assert_eq!(ClubSemRust.get(), true);
        }
    }
}
