#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod reportes_club_sem_rust {
    use registro_de_pagos_club_sem_rust::ClubSemRustRef; // BORRAR LO DE ABAJO Y CAMBIARLO X ESTA LINEA D CODIGO
    #[derive(scale::Decode, scale::Encode,Debug,Clone, PartialEq)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Actividad{
        Futbol,
        Gimnasio,
        Basquet,
        Rugby,
        Hockey,
        Natacion,
        Tenis,
        Paddle
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

        /// Dado un mes, se realiza un Vec de la platita total recaudada de cada categoria
        #[ink(message)]
        pub fn informe_recaudacion_mensual(&self, fecha:Timestamp) ->  Result<Vec<u128>,String>{ // MODIFICAR. TERMINAR. No recibir la fecha, recibir el anio y el mes y chequear que sean validos
            let cant_categorias = self.club_sem_rust.cant_categorias();
            let mut monto_categorias_mensual = vec![0;cant_categorias as usize];
            
            self.club_sem_rust.get_pagos_del_mes(fecha).iter().filter(|p|p.fecha_de_pago().is_some())
            .for_each(|p|{
                let categoria = self.club_sem_rust.categoria_de(p.socio_id()).unwrap(); // CONSULTAR. CONFIRMAR: Para que el pago tenga el id_socio, al crear el pago el socio tiene que existir (por la manera en que se construye el pago)
                monto_categorias_mensual[categoria.discriminant() as usize]+=p.monto();
            
            });
            Ok(monto_categorias_mensual)
        }

        /// Dado un ID_actividad, retorna un listado de IDs de socios no morosos, cuyo plan les permita la asistencia a la actividad dada
        /// ARREGLAR. CONSULTAR. TERMINAR
        #[ink(message)]
        pub fn informe_no_morosos_de_actividad(&self, id_actividad: u32) -> Vec<u32> {//ClubSemRustRef::Actividad) -> Vec<u32> {
            self.club_sem_rust.get_pagos().iter().filter(|p|
                p.fecha_de_pago().is_some() && self.club_sem_rust.socio_tiene_permitida_la_asistencia_a(p.socio_id(),id_actividad).unwrap()) // CONSULTAR. CONFIRMAR: Para que el pago tenga el id_socio, al crear el pago el socio tiene que existir (por la manera en que se construye el pago)
                .map(|p|p.id()).collect()
        } 
    
        fn extra_despues_borrar(){/* es para saber si los warnings se ven o no*/}

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
