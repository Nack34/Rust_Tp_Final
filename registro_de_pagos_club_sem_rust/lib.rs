#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use registro_de_pagos_club_sem_rust::ClubSemRustRef; // CONSULTA: POR QUE NO TODO EL NOMBRE?
#[ink::contract]
mod registro_de_pagos_club_sem_rust {
    use core::iter;

    use datetime::LocalDateTime;
    use ink::storage::Mapping;
    use ink::prelude::string::String;
    use ink::storage::Lazy;

    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Categoria{ 
        A,
        B{id_deporte_seleccionado_por_el_usuario:u32},
        C
    }
    impl Categoria {
        fn discriminant(&self) -> u32 {
            unsafe { *(self as *const Self as *const u32) }
        }
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
    #[derive(scale::Decode, scale::Encode,Debug,Clone)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct DatosCategoria{
        id:u32, // el id es el enum convertido en u32
        nombre:String,
        costo_mensual_en_tokens:u32,
        id_de_actividades_accesibles_base: Vec<u32>,
    }
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Socio{
        id:u32,
        categoria:Categoria,
        datos_personales:DatosPersonalesSocio
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
        
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum TipoId{
        Socio, Actividad, Pago,
    }
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct MappingLens{
        socios:u32,
        actividades:u32,
    }

    #[ink(storage)]
    pub struct ClubSemRust {
        FINALBOSSAccountID:AccountId,
        editores:Mapping<AccountId,AccountId>,
        socios:Mapping<u32,Socio>,
        actividades:Mapping<u32,Actividades>,
        categorias:Mapping<u32,DatosCategoria>,
        pagos:Lazy<Vec<Pago>>, // CONSULTAR 
        mapping_lens:MappingLens,
        cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento:u32,
        porcentaje_de_descuento_por_bonificacion:u32,
        FECHA_DE_HOY_DESPUES_BORRAR:FechaTemporalDespuesBorrar, // borrar, cambia por algun metodo de algun crate que te de la fecha actual
        fecha_de_la_ultima_actualizacion:FechaTemporalDespuesBorrar // no borrar
    }

    impl ClubSemRust {
        #[ink(constructor)]
        pub fn new(FINALBOSSAccountID:AccountId, cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento:u32,mut porcentaje_de_descuento_por_bonificacion:u32)->Self{
            if porcentaje_de_descuento_por_bonificacion > 99 {porcentaje_de_descuento_por_bonificacion = 0} 
            
            let mut csr = Self{FINALBOSSAccountID,
                editores:Mapping::new(),
                socios:Mapping::new(),
                actividades:Mapping::new(),
                categorias:Mapping::new(),
                pagos:Lazy::default(),
                mapping_lens:MappingLens{
                    socios:0,
                    actividades:0,
                 },
                cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento,
                porcentaje_de_descuento_por_bonificacion,
                FECHA_DE_HOY_DESPUES_BORRAR:FechaTemporalDespuesBorrar { mes: 0 },
                fecha_de_la_ultima_actualizacion:FechaTemporalDespuesBorrar { mes: 0 },
                };
            csr.pagos.set(&Vec::new());
            csr
        }

        // ----------- Metodos privados
        fn tiene_permiso(&self) -> bool{
            self.es_FINALBOSS() || self.es_editor()
        }
        fn es_FINALBOSS (&self) -> bool{
            self.FINALBOSSAccountID == self.env().caller()
        }
        fn es_editor (&self) -> bool{
            self.editores.contains(self.env().caller())
        }
        fn nueva_id (&mut self,tipo_id:TipoId) -> u32{
            match tipo_id{
                TipoId::Pago => {return (self.pagos.get_or_default().len()+1) as u32},
                TipoId::Socio => {self.mapping_lens.socios +=1; return self.mapping_lens.socios},
                TipoId::Actividad => {self.mapping_lens.actividades +=1; return self.mapping_lens.actividades},
            }
        }

        fn socio_cumple_las_condiciones_para_obtener_la_bonificacion(&self,socio_id:u32) -> bool{
            let pagos = self.pagos.get_or_default();
            let cant_pagos = pagos.len();
            if cant_pagos < self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento as usize {return false;}

            let mut cant_pagos_del_usuario =0; 
            let mut cant_pagos_del_usuario_que_cumplen_la_condicion =0; 

            for i in (0..cant_pagos).rev() {
                let pago = pagos.get(i).unwrap();
                
                if pago.socio_id==socio_id {
                    cant_pagos_del_usuario+=1;                    
                    if !pago.tiene_bonificacion && !self.pago_esta_vencido(pago){
                        cant_pagos_del_usuario_que_cumplen_la_condicion+=1;
                    } 
                }
                
                if cant_pagos_del_usuario==self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento ||
                cant_pagos_del_usuario>cant_pagos_del_usuario_que_cumplen_la_condicion {break;}
            }
            return cant_pagos_del_usuario_que_cumplen_la_condicion==self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento;
        }
        fn pago_esta_vencido(&self,pago:&Pago) -> bool{
            if let Some(fecha_de_pago) = pago.fecha_de_pago.clone(){
                return fecha_de_pago.mes < pago.fecha_de_vencimiento.mes // aca tiene que ser todo menor, pero en la fecha temporal solo implemente el mes
            }
            false
        }
        fn crear_cuota_para_socio(&mut self, socio_id:u32,fecha_de_vencimiento:FechaTemporalDespuesBorrar) -> bool{
            if !self.socios.contains(socio_id) {return false}
            if fecha_de_vencimiento.mes>self.FECHA_DE_HOY_DESPUES_BORRAR.mes+1 {return false}

            let cumple_las_condiciones_para_obtener_la_bonificacion = self.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id);

            let mut monto_cuota=self.categorias.get::<u32>(self.socios.get(socio_id).unwrap().categoria.discriminant()).unwrap().costo_mensual_en_tokens;
            if cumple_las_condiciones_para_obtener_la_bonificacion {monto_cuota -= monto_cuota*self.porcentaje_de_descuento_por_bonificacion}

            let cuota=Pago{id:self.nueva_id(TipoId::Pago),socio_id,fecha_de_pago:None,
            fecha_de_vencimiento/* datetime::LocalDateTime::now().add_seconds(604800)*/, // reemplazar 604800 por una constante cantidad_segundos_por_dia
            monto:(monto_cuota as u128), 
            tiene_bonificacion:cumple_las_condiciones_para_obtener_la_bonificacion}; 
            
            let mut pagos = self.pagos.get_or_default();
            pagos.push(cuota);
            self.pagos.set(&pagos);
            true
        }

        fn get_socio_dni(&self, un_dni:u32)->Option<Socio>{
            let len = self.mapping_lens.socios;
            for i in 1..len+1{
                let socio = self.socios.get(i).unwrap();
                if socio.datos_personales.dni == un_dni {
                    return Some(socio);
                }
            }
            None
        }
        fn listar_pagos(&self, id_socio:u32) -> Vec<Pago>{
            let pagos = self.pagos.get_or_default();
            let len = pagos.len();
            let mut res = Vec::new();
            for i in 0..len{
                let pago = pagos.get(i).unwrap();
                if pago.socio_id == id_socio {
                    res.push(pago.clone())
                }
            } 
            res
        }
        fn SLICE_O_PAGINACION_QUE_ES_ESO(&self, vec:Vec<Pago>) -> Vec<Pago>{ // CONSULTAR
            vec
        }
        
        fn get_primer_pago_sin_acreditar(&self, socio_id:u32) -> Option<Pago>{
            self.pagos.get_or_default().iter().filter(|p|p.id == socio_id).next().cloned()
        }
        fn marcar_pago_pagado(&mut self, pago_id:u32) -> bool{
            let mut pagos = self.pagos.get_or_default();
            if pagos.len()<pago_id as usize {return false;};

            if let Some(_) = pagos[pago_id as usize].fecha_de_pago.clone(){return false;};

            pagos[pago_id as usize].fecha_de_pago = Some(self.FECHA_DE_HOY_DESPUES_BORRAR.clone());
            self.pagos.set(&pagos);
            
            true
        }


        // ----------- Setters 

        #[ink(message)]
        pub fn set_FINALBOSS (&mut self, nuevo_FINALBOSSAccountID:AccountId) -> bool{
            if self.es_FINALBOSS(){
                self.FINALBOSSAccountID = nuevo_FINALBOSSAccountID; 
                return true;
            }
            false
        }
        #[ink(message)]
        pub fn autorizar_editor (&mut self, nuevo_editor:AccountId) -> bool{
            if self.es_FINALBOSS(){
                self.editores.insert(nuevo_editor.clone(),&nuevo_editor); 
                return true;
            }
            false
        }
        #[ink(message)]
        pub fn desautorizar_editor (&mut self, editor:AccountId) -> bool{
            if self.es_FINALBOSS(){
                self.editores.remove(editor);
                return true;
            }
            false
        }

        #[ink(message)]
        pub fn set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento(&mut self,cant:u32) -> bool{
            if !self.tiene_permiso() {return false;}

            self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento=cant;

            true
        }

        #[ink(message)]
        pub fn registrar_nuevo_socio(&mut self,nombre:String,apellido:String,dni:u32,categoria:Categoria) -> bool{
            if !self.tiene_permiso() {return false;}
            if !self.categorias.contains(categoria.discriminant()) {return false;}

            let info_personal_del_socio=DatosPersonalesSocio{nombre,apellido,dni:dni.clone()};
            let socio=Socio{id:self.nueva_id(TipoId::Socio),categoria,datos_personales:info_personal_del_socio};
            self.socios.insert(socio.id, &socio);

            return self.crear_cuota_para_socio(socio.id, FechaTemporalDespuesBorrar {mes:0}
                                            /* datetime::LocalDateTime::now().add_seconds(604800)*/); 
                                        // reemplazar 604800 por -> 10*(constante cantidad_segundos_por_dia)
        }

        #[ink(message)]
        pub fn actualizacion_mensual(&mut self) -> bool{ // si no se actualiza por varios meses pero SI se crean socios,
                                                        // hay un problema: se le van a agregar cuotas a pagar a los socios nuevos
                                                        // con fecha ANTERIOR a la fecha en la que esos socios se hayan registrado.
                                                        // Una posibilidad es hacer lo que dijo el prof: que este metodo tmb sea llamado
                                                        // cada vez que se realiza un pago, pero si no se realizan pagos x un tiempo,
                                                        // estamos en la misma. Otra solucion que si funcionaria es agregar el campo
                                                        // "fecha_de_registro" a los socios, y cuando se agrega un socio se setea como
                                                        // la fecha actual. Eso funcionaria y solo cambiaria un poco la logica de este metodo // CONSULTAR
            if !self.tiene_permiso() {return false;}
            let meses_desde_la_ultima_actualizacion = self.FECHA_DE_HOY_DESPUES_BORRAR.mes - self.fecha_de_la_ultima_actualizacion.mes;
            if !(meses_desde_la_ultima_actualizacion>0) {return false;}

            for mes in self.fecha_de_la_ultima_actualizacion.mes+1..self.FECHA_DE_HOY_DESPUES_BORRAR.mes+1 { // el ultimo mes registrado no quiero repetirlo y el mes actual si quiero registrarlo 
                for i in 1..self.mapping_lens.socios+1 {
                    self.crear_cuota_para_socio(i, FechaTemporalDespuesBorrar {mes}
                                            /* datetime::LocalDateTime::mes().add_seconds(604800)*/); 
                                        // reemplazar 604800 por -> 10*(constante cantidad_segundos_por_dia)
                }
            }
            self.fecha_de_la_ultima_actualizacion = self.FECHA_DE_HOY_DESPUES_BORRAR.clone();
            true
        }

        #[ink(message)]
        pub fn registrar_nuevo_pago(&mut self, dni_socio:u32, monto:u32 ) -> bool{
            if !self.tiene_permiso() {return false;}
            let Some(socio) = self.get_socio_dni(dni_socio) else {return false;};
            
            let Some(pago) = self.get_primer_pago_sin_acreditar(socio.id) else {return false;};
            if !(pago.monto == monto as u128) {return false;};

            self.marcar_pago_pagado(pago.id)

            // CONSULTA: Tomamos como que el monto entrante ya le fue descontado al socio? Que pasa si no tiene deuda o si el monto es mayor? Pierde
            // plata? NO SABEMOSSSSSSSSSSSSSSSSS
            
        }

        // ----------- Getters

        #[ink(message)]
        pub fn soy_FINNALBOSS(&self) -> bool{
            self.es_FINALBOSS()
        }
        #[ink(message)]
        pub fn puedo_editar(&self) -> bool{
            self.tiene_permiso()
        }

        #[ink(message)]
        pub fn get_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento(&mut self,cant:u32)->u32{
            self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento
        }

        #[ink(message)]
        pub fn consulta_de_pago(&self, dni_ingresado:Option<u32>)->Option<Vec<Pago>>{ 
            if !self.tiene_permiso() {return None;}
            let Some(dni_ingresado) = dni_ingresado else {return Some(self.SLICE_O_PAGINACION_QUE_ES_ESO(self.pagos.get_or_default())); }; //CONSULTA: tambien podria ser self.pagos.get();
            let Some(socio) = self.get_socio_dni(dni_ingresado) else {return None;};
            let pagos_de_un_socio=self.listar_pagos(socio.id);
            Some (self.SLICE_O_PAGINACION_QUE_ES_ESO(pagos_de_un_socio))
        }

        #[ink(message)]
        pub fn get_pagos(&self) -> Vec<Pago>{
            self.pagos.get_or_default()
        }
        
        #[ink(message)]
        pub fn get_categoria_data(&self,categoria:Categoria) -> DatosCategoria{
            self.categorias.get(categoria.discriminant()).unwrap().clone() // SI O SI, SI EXISTE LA CATEGORIA TENDRIA Q EXISTIR SU DATA. HACERLO EN EL CONSTRUCTOR? SINO DEVOLVER UN OPTION Y LISTO. TERMINAR
        }

        #[ink(message)]
        pub fn categoria_de(&self,socio_id:u32)->u32{0} // CONSULTAR: ESTO ESTA BIEN? ESTAS ACCEDIENDO A LA PRIVACIDAD DEL SOCIO PARA OBTENER SUS DATOS

    }

    // CONSULTAS: 
        // 1- BOOOOOOOOOOOOOOL PORFAVORRRRRRRRRRRRR
        // 2- Metodo actualizacion_mensual
        // 3- Fecha? Que hacemos? "the trait bound `LocalDateTime: TypeInfo` is not satisfied"
        // 4- Metodo registrar_nuevo_pago
    
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
    }
}
