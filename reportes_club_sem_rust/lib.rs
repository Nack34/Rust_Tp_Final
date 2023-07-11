#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod reportes_club_sem_rust {
    use registro_de_pagos_club_sem_rust::ClubSemRustRef;
    
    /// Los posibles tipos de errores all llamar a los metodos del contrato
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Devuelto si el Id de actividad ingresado no representa una actividad del Club.
        ActividadInvalida,
        /// Devuelto si el no todas las categorias del Club tienen data.
        NoTodasLasCategoriasTienenData,
        /// Devuelto si el AccountId que llama no tiene permitido realizar la operacion, puesto que no posee los permisos necesarios.
        NoSePoseenLosPermisosSuficientes,
    } 
    
    #[ink(storage)]
    pub struct ReportesClubSemRust {
        club_sem_rust: ClubSemRustRef,
    }

    impl ReportesClubSemRust {
        #[ink(constructor)]
        pub fn new(club_sem_rust: ClubSemRustRef) -> Self {
            Self { club_sem_rust }
        }

        /// Se realiza un Vec de id_de_usuarios agregando aquellos socios morosos del Club
        #[ink(message)]
        pub fn verificacion_de_pagos_pendientes(&self) -> Result<Vec<u32>,Error>{
            if !self.club_sem_rust.tengo_permisos_suficientes_para_realizar_operaciones() {return Err(Error::NoSePoseenLosPermisosSuficientes)};
            let ids = self.club_sem_rust.get_pagos().unwrap().iter().filter(|p|p.fecha_de_pago().is_none()).map(|p|p.id()).collect();
            Ok(ids)
        }

        /// Dado un Timestamp, se realiza un Vec de la platita total recaudada de cada categoria en ese mes y anio
        #[ink(message)]
        pub fn informe_recaudacion_mensual(&self, mes_y_anio:Timestamp) -> Result<Vec<u128>,Error>{
            if !self.club_sem_rust.tengo_permisos_suficientes_para_realizar_operaciones() {return Err(Error::NoSePoseenLosPermisosSuficientes)};
            let cant_categorias = self.club_sem_rust.cant_categorias();
            let mut monto_categorias_mensual = vec![0;cant_categorias as usize];
            
            self.club_sem_rust.get_pagos_del_mes_y_anio(mes_y_anio).unwrap().iter().filter(|p|p.fecha_de_pago().is_some())
            .for_each(|p|{
                let categoria = self.club_sem_rust.categoria_de(p.socio_id()).unwrap();
                monto_categorias_mensual[categoria.discriminant() as usize]+=p.monto();
            
            });
            Ok(monto_categorias_mensual)
        }

        /// Dado un ID_actividad, retorna un listado de IDs de socios no morosos, cuyo plan les permita la asistencia a la actividad dada
        /// 
        /// Posibles Error: ActividadInvalida, NoTodasLasCategoriasTienenData
        #[ink(message)]
        pub fn informe_no_morosos_de_actividad(&self, id_actividad: u32) -> Result<Vec<u32>,Error> {
            if !self.club_sem_rust.tengo_permisos_suficientes_para_realizar_operaciones() {return Err(Error::NoSePoseenLosPermisosSuficientes)};
            if !self.club_sem_rust.existe_actividad_id(id_actividad) {return Err(Error::ActividadInvalida)}
            if !self.club_sem_rust.todas_las_categorias_tienen_sus_datas_cargadas() {return Err(Error::NoTodasLasCategoriasTienenData)}
            
            let mut res= self.club_sem_rust.get_pagos().unwrap().iter().filter(|p|
                p.fecha_de_pago().is_some() && self.club_sem_rust.socio_tiene_permitida_la_asistencia_a(p.socio_id(),id_actividad).unwrap()) 
                .map(|p|p.id()).collect::<Vec<u32>>();
            res.dedup();
            Ok(res)
        } 
    }




    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use core::hash::Hash;

        use super::*;

        fn crear_contrato() {/*-> ReportesClubSemRust{
            let contract = ink::env::account_id::<ink::env::DefaultEnvironment>();
            ink::env::test::set_callee::<ink::env::DefaultEnvironment>(contract);
            
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let duenio = accounts.alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(duenio);
            
            let club_ref=ClubSemRustRef::new(duenio,3,10,true);
            club_ref.;
            //let rep_club=ReportesClubSemRust::new(club_ref);
            ReportesClubSemRust::new()*/
        } 
        #[ink::test]
        fn test_verificacion_de_pagos_pendientes() {
            
        }
    }

}
