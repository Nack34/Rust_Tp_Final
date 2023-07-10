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
        pub fn verificacion_de_pagos_pendientes(&self) -> Vec<u32>{
            self.club_sem_rust.get_pagos().iter().filter(|p|p.fecha_de_pago().is_none()).map(|p|p.id()).collect()
        }

        /// Dado un Timestamp, se realiza un Vec de la platita total recaudada de cada categoria en ese mes y anio
        #[ink(message)]
        pub fn informe_recaudacion_mensual(&self, mes_y_anio:Timestamp) ->  Vec<u128>{
            let cant_categorias = self.club_sem_rust.cant_categorias();
            let mut monto_categorias_mensual = vec![0;cant_categorias as usize];
            
            self.club_sem_rust.get_pagos_del_mes_y_anio(mes_y_anio).iter().filter(|p|p.fecha_de_pago().is_some())
            .for_each(|p|{
                let categoria = self.club_sem_rust.categoria_de(p.socio_id()).unwrap();
                monto_categorias_mensual[categoria.discriminant() as usize]+=p.monto();
            
            });
            monto_categorias_mensual
        }

        /// Dado un ID_actividad, retorna un listado de IDs de socios no morosos, cuyo plan les permita la asistencia a la actividad dada
        /// 
        /// Posibles Error: ActividadInvalida, NoTodasLasCategoriasTienenData
        #[ink(message)]
        pub fn informe_no_morosos_de_actividad(&self, id_actividad: u32) -> Result<Vec<u32>,Error> {
            if !self.club_sem_rust.existe_actividad_id(id_actividad) {return Err(Error::ActividadInvalida)}
            if !self.club_sem_rust.todas_las_categorias_tienen_sus_datas_cargadas() {return Err(Error::NoTodasLasCategoriasTienenData)}
            
            let mut res= self.club_sem_rust.get_pagos().iter().filter(|p|
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
