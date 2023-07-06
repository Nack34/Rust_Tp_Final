#![cfg_attr(not(feature = "std"), no_std, no_main)]

/* en cargo.toml tiene que ir:

registro_de_pagos_club_sem_rust = { path = "registro_de_pagos_club_sem_rust" /* COMO SE PONE EL PATH??? */, default-features = false, features = ["ink-as-dependency"] }

std = [
    "ink/std",
    
    //COSAS
    
    "registro_de_pagos_club_sem_rust/std", //ACA VA EL PATH O QUE COSA? QUE ES STD
]

 */


#[ink::contract]
mod reportes_club_sem_rust {
    //use registro_de_pagos_club_sem_rust::ClubSemRustRef; // BORRAR LO DE ABAJO Y CAMBIARLO X ESTA LINEA D CODIGO

    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct ClubSemRustRef{}
    impl ClubSemRustRef {
        //setters
        pub fn set_FINALBOSS (&mut self, nuevo_FINALBOSSAccountID:AccountId) -> bool{false}
        pub fn autorizar_editor (&mut self, nuevo_editor:AccountId) -> bool{false}
        pub fn desautorizar_editor (&mut self, editor:AccountId) -> bool{false}
        pub fn set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento(&mut self,cant:u32) -> bool{false}
        pub fn registrar_nuevo_socio(&mut self,nombre:String,apellido:String,dni:u32,categoria_id:u32) -> bool{false}
        pub fn actualizacion_mensual(&mut self) -> bool{false}
        pub fn registrar_nuevo_pago(&mut self, dni_socio:u32, monto:u32 ) -> bool{false}
        //getters
        pub fn soy_FINNALBOSS(&self) -> bool{false}
        pub fn puedo_editar(&self) -> bool{false}
        pub fn get_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento(&mut self,cant:u32)->u32{0}
        pub fn consulta_de_pago(&self, dni_ingresado:Option<u32>)->Option<Vec<Pago>>{None}
        pub fn get_pagos(&self) -> Vec<Pago>{Vec::new()}
        pub fn categoria_de(&self,socio_id:u32)->u32{0}
    }
    #[derive(scale::Decode, scale::Encode,Debug,Clone)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Pago{
        id:u32,
        socio_id: u32,
        fecha_de_pago:Option<FechaTemporalDespuesBorrar>, //Option<datetime::LocalDateTime>, //pa pregunta
        fecha_de_vencimiento:FechaTemporalDespuesBorrar, //datetime::LocalDateTime,
        monto:u128,
        tiene_bonificacion:bool
    }
    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct FechaTemporalDespuesBorrar{
        mes:u32
    }



    // VOLAR TODO LO DE ARRIBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA


/* 


 */


    
    #[ink(storage)]
    pub struct ReportesClubSemRust {
        club_sem_rust: ClubSemRustRef,
    }

    impl ReportesClubSemRust {
        #[ink(constructor)]
        pub fn new(club_sem_rust: ClubSemRustRef) -> Self {
            Self { club_sem_rust }
        }

        #[ink(message)]
        /// Se realiza un Vec de id_de_usuarios agregando aquellos socios morosos del Vec<Pagos>
        pub fn verificacion_de_pagos_pendientes(&self) -> Vec<u32>{
            self.club_sem_rust.get_pagos().iter().filter(|p|p.fecha_de_pago.is_none()).map(|p|p.id).collect()
        }

        #[ink(message)]
        // CONSULTA: ¿el informe es del mes anterior o de todos los meses de los que se tiene registro? // UN SOLO MES
        /// Se realiza un Vec de platita de cada categoria de cada mes
        pub fn informe_recaudacion_mensual(&self) ->  Vec<Vec<u128>>{
            let cant_meses = 15; // CONSULTA: COMO SABEMOS LA CANT DE MESES???
            let cant_categorias = 5; // CONSULTA: COMO SABEMOS LA CANT DE categorias???
            let mut monto_categorias_mensual = vec![vec![0;cant_meses];cant_categorias];
            
            self.club_sem_rust.get_pagos().iter().filter(|p|p.fecha_de_pago.is_some())
            .for_each(|p|
                monto_categorias_mensual[self.club_sem_rust.categoria_de(p.socio_id) as usize][p.fecha_de_pago.clone().unwrap().mes as usize]+=p.monto
            );
            monto_categorias_mensual
        }

        #[ink(message)]
        /// Dado un ID_actividad, retorna un listado de IDs de socios no morosos, cuyo plan les permita la asistencia a la actividad dada
        pub fn informe_no_morosos_de_actividad(&self, id_actividad: u32) -> Vec<u32> {
            self.club_sem_rust.get_pagos().iter().filter(|p|
                p.fecha_de_pago.is_some() && self.socio_tiene_permitida_la_asistencia_a(p.socio_id,id_actividad))
                .map(|p|p.id).collect()
        } 
        
        fn socio_tiene_permitida_la_asistencia_a(&self, socio_id:u32,id_actividad:u32)->bool{ // CONSULTAR: medio feo quedo. Esta bien asi? 
            /*
            let categoria = self.club_sem_rust.socios.get(socio_id).categoria.clone();

            let mut id_deporte_seleccionado_por_el_usuario_ES = false;
            match categoria{
                ClubSemRustRef::Categoria::B{id_deporte_seleccionado_por_el_usuario} => {id_deporte_seleccionado_por_el_usuario_ES= id_deporte_seleccionado_por_el_usuario == id_actividad}
                _ => {}
            }
            if id_deporte_seleccionado_por_el_usuario_ES {return id_deporte_seleccionado_por_el_usuario_ES}

            let cant = self.club_sem_rust.get_categoria(categoria).id_de_actividades_accesibles_base.iter().filter(|a|a == id_actividad.discriminant()).count(); // obtengo la cantidad de actividades que se coincidan con la actividad que viene como parametro
            return cant>0; // o es 0 o es 1 (supuestamente)
            
            */
            true
        }

    }




    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /*/// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let reportes_club_sem_rust = ReportesClubSemRust::default();
            assert_eq!(reportes_club_sem_rust.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut reportes_club_sem_rust = ReportesClubSemRust::new(false);
            assert_eq!(reportes_club_sem_rust.get(), false);
            reportes_club_sem_rust.flip();
            assert_eq!(reportes_club_sem_rust.get(), true);
        }*/
    }

}
