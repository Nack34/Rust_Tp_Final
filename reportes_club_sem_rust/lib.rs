#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod reportes_club_sem_rust {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    use registro_de_pagos_club_sem_rust::ClubSemRustRef;
    /// Los posibles tipos de errores al llamar a los metodos del contrato
    /// 
    /// ActividadInvalida es devuelto si el Id de actividad ingresado no representa una actividad del Club.
    /// 
    /// NoTodasLasCategoriasTienenData es devuelto si el no todas las categorias del Club tienen data.
    /// 
    /// NoSePoseenLosPermisosSuficientes es devuelto si el AccountId que llama no tiene permitido realizar la operacion, puesto que no posee los permisos necesarios.
    /// 
    /// FechaInvalida es devuelto si la fecha ingresada no corresponde a una fecha real
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Devuelto si el Id de actividad ingresado no representa una actividad del Club.
        ActividadInvalida,
        /// Devuelto si el no todas las categorias del Club tienen data.
        NoTodasLasCategoriasTienenData,
        /// Devuelto si el AccountId que llama no tiene permitido realizar la operacion, puesto que no posee los permisos necesarios.
        NoSePoseenLosPermisosSuficientes,
        /// Devuelto si la fecha ingresada no corresponde a una fecha real
        FechaInvalida,
    }
    #[ink(storage)]
    pub struct ReportesClubSemRust {
        #[cfg(not(test))] 
        club_sem_rust: ClubSemRustRef,
    }

    impl ReportesClubSemRust {
        #[cfg(not(test))] 
        #[ink(constructor)]
        pub fn new(club_sem_rust: ClubSemRustRef) -> Self {
            Self { club_sem_rust }
        }

        /// Se realiza un Vec de id_de_usuarios agregando aquellos socios morosos del Club
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes
        #[cfg(not(test))] 
        #[ink(message)]
        pub fn verificacion_de_pagos_morosos(&self) -> Result<Vec<u128>,Error>{
            if !self.club_sem_rust.tengo_permisos_suficientes_para_realizar_operaciones() {return Err(Error::NoSePoseenLosPermisosSuficientes)};
            let ids = self.club_sem_rust.get_data_pagos().unwrap().iter().filter(|p|self.club_sem_rust.pago_esta_vencido(p.0).unwrap()).map(|p|p.1).collect();
            Ok(ids)
        }

        /// Dado un mes y un anio, se realiza un Vec de la platita total recaudada de cada categoria en ese mes y anio
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, FechaInvalida
        #[cfg(not(test))] 
        #[ink(message)]
        pub fn informe_recaudacion_mensual(&self, mes:i8, anio:i64) -> Result<Vec<u128>,Error>{
            if !self.club_sem_rust.tengo_permisos_suficientes_para_realizar_operaciones() {return Err(Error::NoSePoseenLosPermisosSuficientes)};
            
            let binding = self.club_sem_rust.get_data_pagos_del_mes_y_anio(mes, anio);
            let Ok(pagos) = binding else {return Err(Error::FechaInvalida)};

            let cant_categorias = self.club_sem_rust.cant_categorias();
            let mut monto_categorias_mensual = Vec::new();
            for _ in 0..cant_categorias{ monto_categorias_mensual.push(0); }

            // para tomarlo como recaudado, los pagod tienen que estar pagados
            pagos.iter().filter(|p|p.2.is_some()).for_each(|p|{
                let categoria = self.club_sem_rust.get_data_socio_con_id(p.1).unwrap().4;
                monto_categorias_mensual[self.club_sem_rust.get_categoria_id(categoria) as usize]+=p.4;
            });
            Ok(monto_categorias_mensual)
        }

        /// Dado un ID_actividad, retorna un listado de IDs de socios no morosos, cuyo plan les permita la asistencia a la actividad dada
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, ActividadInvalida, NoTodasLasCategoriasTienenData
        #[cfg(not(test))] 
        #[ink(message)]
        pub fn informe_no_morosos_de_actividad(&self, id_actividad: u32) -> Result<Vec<u128>,Error> {
            if !self.club_sem_rust.tengo_permisos_suficientes_para_realizar_operaciones() {return Err(Error::NoSePoseenLosPermisosSuficientes)};
            if !self.club_sem_rust.existe_actividad_con_id(id_actividad) {return Err(Error::ActividadInvalida)}
            if !self.club_sem_rust.todas_las_categorias_tienen_sus_datas_cargadas() {return Err(Error::NoTodasLasCategoriasTienenData)}
            
            let mut res= self.club_sem_rust.get_data_pagos().unwrap().iter().filter(|p|
                !self.club_sem_rust.pago_esta_vencido(p.0).unwrap() && self.club_sem_rust.socio_tiene_permitida_la_asistencia_a(p.1,id_actividad).unwrap()) 
                .map(|p|p.0).collect::<Vec<u128>>();
            res.dedup();
            Ok(res)
        } 
    }







// ---------------------- De aca para abajo solo esta para hacer test. Se creo un "ambiente falso" para poder testear el contrato ----------------------
// ---------------------- El "ambiente falso" se llama ClubSemRustFake, y representa a ClubSemRust, con sus metodos retornando valores para testing ----------------------
// ---------------------- Todos los metodos de ReportesClubSemRustFake tienen exactamente la misma logica que los de ReportesClubSemRust ----------------------

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[repr(u32)]
    pub enum Actividad{
        Futbol,
        Gimnasio,
        Basquet,
        Rugby,
        Hockey,
        Natacion,
        Tenis,
        Paddle,
    }
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[repr(u32)]
    pub enum Categoria{ 
        A,
        B{deporte_seleccionado_por_el_usuario:Actividad},
        C,
    }
    impl Categoria{
        #[cfg(test)] 
        fn discriminant(&self) -> u32 {
            unsafe { *<*const _>::from(self).cast::<u32>() }
        }
    }
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    struct ClubSemRustFake{}
    impl ClubSemRustFake{
        #[cfg(test)] 
        pub fn new()->Self{
            Self {}
        }
        #[cfg(test)] 
        pub fn tengo_permisos_suficientes_para_realizar_operaciones(&self)->bool{
            true
        }
        #[cfg(test)] 
        fn cant_categorias(&self)->u32{
            3
        }
        #[cfg(test)] 
        pub fn existe_actividad_con_id (&self, _id_actividad:u32) -> bool{
            true
        }
        #[cfg(test)] 
        pub fn todas_las_categorias_tienen_sus_datas_cargadas(&self) ->bool{
            true
        }
        #[cfg(test)] 
        pub fn socio_tiene_permitida_la_asistencia_a(&self, _socio_id:u128,_id_actividad: u32) -> Result<bool,Error>{
            Ok(true)
        }
        #[cfg(test)] 
        pub fn get_data_socio_con_id(&self,_socio_id:u128)->Result<(String,String,u32,u128,Categoria),Error>{
            Ok((String::default(),String::default(),u32::default(),u128::default(),Categoria::C))
        }
        /// Retorna todos datos de los pagos que se han realizado al Club
        /// 
        /// La informacion del pago se retornara en el siguiente formato: 
        /// 
        /// 0: id unico que tiene en el club
        /// 
        /// 1: socio_id
        /// 
        /// 2: fecha_de_pago
        /// 
        /// 3: fecha_de_vencimiento
        /// 
        /// 4: monto
        /// 
        /// 5: pago tiene bonificacion
        #[cfg(test)] 
        pub fn get_data_pagos(&self) -> Result<Vec<(u128,u128,Option<Timestamp>,Timestamp,u128,bool)>,Error>{
            Ok(vec![(0,9,None,800,100,false),(1,10,Some(700),700,101,true), //
                    (2,11,None,700,102,false),(3,12,Some(800),700,103,true),
                    (4,13,None,600,104,false),(5,14,Some(600),700,105,true),
                    (6,15,None,700,106,false),(7,16,Some(700),700,107,true)])
        }
        #[cfg(test)] 
        pub fn get_data_pagos_del_mes_y_anio(&self, _mes:i8, _anio:i64) -> Result<Vec<(u128,u128,Option<Timestamp>,Timestamp,u128,bool)>,Error>{
            Ok(vec![(0,4,None,800,100,false),(1,5,Some(700),700,101,true), //
                    (2,6,None,700,102,false),(3,7,Some(800),700,103,true)])
        } 
        #[cfg(test)] 
        pub fn pago_esta_vencido (&self,pago_id:u128) -> Result<bool,Error>{
            let pago_id = pago_id as usize;
            let pagos = self.get_data_pagos().unwrap();

            let pago = &pagos[pago_id];
            
            if let Some(fecha_de_pago) = pago.2.clone(){
                return Ok(fecha_de_pago > pago.3)
            }
            // decimos que "ahora" es 700, para simplificar el testeo
            return Ok(700 > pago.3);
        }

        #[cfg(test)] 
        pub fn get_categoria_id (&self, categoria:Categoria) -> u32{
            categoria.discriminant()
        }
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    struct ReportesClubSemRustFake {
        #[cfg(test)] 
        club_sem_rust: ClubSemRustFake,
    }
    impl ReportesClubSemRustFake{
        #[cfg(test)] 
        pub fn new(club_sem_rust: ClubSemRustFake) -> Self {
            Self { club_sem_rust }
        }

        /// Copia para testear
        #[cfg(test)] 
        pub fn verificacion_de_pagos_morosos(&self) -> Result<Vec<u128>,Error>{
            if !self.club_sem_rust.tengo_permisos_suficientes_para_realizar_operaciones() {return Err(Error::NoSePoseenLosPermisosSuficientes)};
            let ids = self.club_sem_rust.get_data_pagos().unwrap().iter().filter(|p|self.club_sem_rust.pago_esta_vencido(p.0).unwrap()).map(|p|p.1).collect();
            Ok(ids)
        }

        /// Copia para testear
        #[cfg(test)] 
        pub fn informe_recaudacion_mensual(&self, mes:i8, anio:i64) -> Result<Vec<u128>,Error>{
            if !self.club_sem_rust.tengo_permisos_suficientes_para_realizar_operaciones() {return Err(Error::NoSePoseenLosPermisosSuficientes)};
            
            let binding = self.club_sem_rust.get_data_pagos_del_mes_y_anio(mes, anio);
            let Ok(pagos) = binding else {return Err(Error::FechaInvalida)};

            let cant_categorias = self.club_sem_rust.cant_categorias();
            let mut monto_categorias_mensual = vec![0;cant_categorias as usize];

            // para tomarlo como recaudado, los pagos tienen que estar pagados
            pagos.iter().filter(|p|p.2.is_some()).for_each(|p|{
                let categoria = self.club_sem_rust.get_data_socio_con_id(p.1).unwrap().4;
                monto_categorias_mensual[self.club_sem_rust.get_categoria_id(categoria) as usize]+=p.4;
            });
            Ok(monto_categorias_mensual)
        }

        /// Copia para testear
        #[cfg(test)] 
        pub fn informe_no_morosos_de_actividad(&self, id_actividad: u32) -> Result<Vec<u128>,Error> {
            if !self.club_sem_rust.tengo_permisos_suficientes_para_realizar_operaciones() {return Err(Error::NoSePoseenLosPermisosSuficientes)};
            if !self.club_sem_rust.existe_actividad_con_id(id_actividad) {return Err(Error::ActividadInvalida)}
            if !self.club_sem_rust.todas_las_categorias_tienen_sus_datas_cargadas() {return Err(Error::NoTodasLasCategoriasTienenData)}
            
            let mut res= self.club_sem_rust.get_data_pagos().unwrap().iter().filter(|p|
                !self.club_sem_rust.pago_esta_vencido(p.0).unwrap() && self.club_sem_rust.socio_tiene_permitida_la_asistencia_a(p.1,id_actividad).unwrap()) 
                .map(|p|p.1).collect::<Vec<u128>>();
            res.dedup();
            Ok(res)
        } 
    }


    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        use super::*;
        #[ink::test]
        fn test_verificacion_de_pagos_pendientes() {
            let club=ClubSemRustFake::new();
            let reportes=ReportesClubSemRustFake::new(club);
            let vec=reportes.verificacion_de_pagos_morosos().unwrap();
            let vec_manual=vec![12,13];
            assert_eq!(vec_manual,vec);
        }
        #[ink::test]
        fn informe_recaudacion_mensual(){
            let club=ClubSemRustFake::new();
            let reportes=ReportesClubSemRustFake::new(club);
            let vec=reportes.informe_recaudacion_mensual(1, 1).unwrap();
            let vec_manual=vec![0,0,204];
            assert_eq!(vec,vec_manual);
        }
        #[ink::test]
        fn informe_no_morosos_de_actividad(){
            let club=ClubSemRustFake::new();
            let reportes=ReportesClubSemRustFake::new(club);
            let vec=reportes.informe_no_morosos_de_actividad(1).unwrap();
            let vec_manual=vec![9,10,11,14,15,16];
            assert_eq!(vec,vec_manual);
        }
    }

}
