#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod Rust_Tp_Final {
    use datetime::LocalDateTime;
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    // no es mejor usar #[ink::storage_item] ¿? segun el ink implementa todos los traits
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
        apellido:String,
        dni:u32
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
        fecha_de_pago:Option<datetime::LocalDateTime>,
        fecha_de_vencimiento:datetime::LocalDateTime,
        monto:u128,
        pago_con_bonificacion:bool
    }
    #[ink(storage)]
    pub struct ClubSemRust {
        socios:Mapping<u32,Socio>,
        actividades:Mapping<u32,Actividades>,
        categorias:Mapping<u32,Categoria>,
        cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento:u32,
        pagos:Vec<Pago>
    }

    impl ClubSemRust {
        #[ink(constructor)]
        pub fn new(cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento:u32)->Self{
            Self{socios:Mapping::new(),
                actividades:Mapping::new(),
                categorias:Mapping::new(),
                cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento,
                pagos:Vec::new()
            }
        }
        #[ink(message)]
        pub fn set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento(&mut self,cant:u32){
            self.cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento=cant;
        }
        #[ink(message)]
        pub fn get_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento(&mut self,cant:u32)->u32{
            self.cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento
        }

        #[ink(message)]
        pub fn registrar_nuevo_socio(&mut self,nombre:String,apellido:String,dni:u32,categoria_id:u32){
            // se utiliza la longitud del mapping +1 para el id del socio pero como no pude utilizo el dni
            let info_personal_del_socio=DatosPersonalesSocio{nombre,apellido,dni:dni.clone()};
            let socio=Socio{id:dni,categoria_id,datos_personales:info_personal_del_socio};
            self.socios.insert(socio.id, &socio);
            let monto=self.categorias.get(categoria_id).unwrap().costo_mensual_en_tokens;
            let pago=Pago{id:socio.id,fecha_de_pago:None,fecha_de_vencimiento:datetime::LocalDateTime::now().add_seconds(604800),monto,pago_con_bonificacion:false};//que monto ponemos?
        }

    }
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
    }
}
