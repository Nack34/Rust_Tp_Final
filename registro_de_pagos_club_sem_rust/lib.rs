#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use registro_de_pagos_club_sem_rust::ClubSemRustRef; // CONSULTA: POR QUE NO TODO EL NOMBRE?
#[ink::contract]
mod registro_de_pagos_club_sem_rust {

    use datetime::LocalDateTime;

    use ink::storage::Mapping;
    use ink::prelude::string::String;
    use ink::storage::Lazy;

    
    /// Voy a llorar del asco que me da hacer "Cantidad". Pido disculpas por los malestares emocionales que esto pueda llegar a crear
    /// "Cantidad"  puede dar errores
    /// MODIFICAR. TERMINAR. PEDIR DISCULPAS 
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Categoria{ 
        A,
        B{id_deporte_seleccionado_por_el_usuario:u32},
        C,
        Cantidad, 
    }
    impl Categoria {
        fn discriminant(&self) -> u32 {
            unsafe { *(self as *const Self as *const u32) }
        }
    }
    #[derive(scale::Decode, scale::Encode,Debug,Clone)]
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
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    struct DatosPersonalesSocio{
        nombre:String,
        apellido:String,
        dni:u32
    }

    /// DOCUMENTAR. TERMINAR
    /// 
    /// 
    /// 
    /// 
    /// 
    /// 
    #[derive(scale::Decode, scale::Encode,Debug,Clone)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct DatosCategoria{
        id:u32, // el id es el enum convertido en u32
        nombre:String,
        costo_mensual_en_tokens:u32,
        actividades_accesibles_base: Vec<Actividad>, // CONSULTAR: No se puede HashSet?. // the trait `TypeInfo` is not implemented for `HashSet<Actividad>` // the trait bound `Mapping<Actividad, Actividad>: WrapperTypeDecode` is not satisfied
    }

    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    struct Socio{
        id:u32,
        categoria:Categoria,
        datos_personales:DatosPersonalesSocio
    }

    /// DOCUMENTAR. TERMINAR
    /// 
    /// 
    /// 
    /// 
    /// 
    #[derive(scale::Decode, scale::Encode,Debug,Clone)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Pago{
        id:u32,
        socio_id: u32,
        fecha_de_pago:Option<Timestamp>,
        fecha_de_vencimiento:Timestamp,
        monto:u128,
        tiene_bonificacion:bool
    }
        
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    enum TipoId{
        Socio, Pago,
    }
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    struct MappingLens{
        socios:u32,
    }

    #[ink(storage)]
    pub struct ClubSemRust {
        politica_de_autorizacion_activada:bool,
        duenio_account_id:AccountId,
        editores:Mapping<AccountId,AccountId>,
        socios:Mapping<u32,Socio>,
        categorias_data:Mapping<u32,DatosCategoria>,
        pagos:Lazy<Vec<Pago>>, // CONSULTAR 
        mapping_lens:MappingLens,
        cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento:u32,
        porcentaje_de_descuento_por_bonificacion:u32,
        fecha_de_la_ultima_actualizacion:Timestamp // no borrar
    }

    impl ClubSemRust {
        #[ink(constructor)]
        pub fn new(duenio_account_id:AccountId, cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento:u32,mut porcentaje_de_descuento_por_bonificacion:u32)->Self{
            if porcentaje_de_descuento_por_bonificacion > 99 {porcentaje_de_descuento_por_bonificacion = 0} 
            

            let mut csr = Self{
                politica_de_autorizacion_activada:true,
                duenio_account_id,
                editores:Mapping::new(),
                socios:Mapping::new(),
                categorias_data:Mapping::new(),
                pagos:Lazy::default(),
                mapping_lens:MappingLens{
                    socios:0,
                },
                cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento,
                porcentaje_de_descuento_por_bonificacion,
                fecha_de_la_ultima_actualizacion:Timestamp::default(),
                };
            csr.fecha_de_la_ultima_actualizacion = csr.env().block_timestamp(); 
            csr.pagos.set(&Vec::new());
            csr
        }

        // ----------- Metodos privados
        
        fn tiene_permiso(&self) -> bool{
            self.politica_de_autorizacion_activada || self.es_duenio() || self.es_editor()
        }
        fn es_duenio (&self) -> bool{
            self.duenio_account_id == self.env().caller()
        }
        fn es_editor (&self) -> bool{
            self.editores.contains(self.env().caller())
        }

        fn nueva_id (&mut self,tipo_id:TipoId) -> u32{
            match tipo_id{
                TipoId::Pago => {return (self.pagos.get_or_default().len()+1) as u32},
                TipoId::Socio => {self.mapping_lens.socios +=1; return self.mapping_lens.socios},
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
                return LocalDateTime::at(fecha_de_pago as i64) < pago.fecha_de_vencimiento.mes // aca tiene que ser todo menor, pero en la fecha temporal solo implemente el mes
            }
            false
        }
/*
        fn crear_cuota_para_socio(&mut self, socio_id:u32,fecha_de_vencimiento:FechaTemporalDespuesBorrar) -> Result<String,String>{
            if !self.socios.contains(socio_id) {return Err("el socio no posee los permisos para realizar esta operacion".to_string())}
            if fecha_de_vencimiento.mes>self.FECHA_DE_HOY_DESPUES_BORRAR.mes+1 {return Err(" la fecha de vencimiento recibida por parametro es invalida".to_string())}

            let cumple_las_condiciones_para_obtener_la_bonificacion = self.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id);

            let mut monto_cuota=self.categorias_data.get::<u32>(self.socios.get(socio_id).unwrap().categoria.discriminant()).unwrap().costo_mensual_en_tokens;
            if cumple_las_condiciones_para_obtener_la_bonificacion {monto_cuota -= monto_cuota*self.porcentaje_de_descuento_por_bonificacion}

            let cuota=Pago{id:self.nueva_id(TipoId::Pago),socio_id,fecha_de_pago:None,
            fecha_de_vencimiento/* datetime::LocalDateTime::now().add_seconds(604800)*/, // reemplazar 604800 por una constante cantidad_segundos_por_dia
            monto:(monto_cuota as u128), 
            tiene_bonificacion:cumple_las_condiciones_para_obtener_la_bonificacion}; 
            
            let mut pagos = self.pagos.get_or_default();
            pagos.push(cuota);
            self.pagos.set(&pagos);
            Ok("se realizo la operacion exitosamente".to_string())
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

        /// Dado un AccountId, lo setea como nuevo duenio del Club
        /// Posibles result:
        #[ink(message)]
        pub fn set_duenio (&mut self, nuevo__auenioAccount_id:AccountId) -> Result<String,String>{ //TERMINAR. Devolver Result
            if self.es_duenio(){
                self.duenio_account_id = nuevo__auenioAccount_id; 
                return Ok("la reasignacion del id del dueño se realizo exitosamente".to_string());
            }
            Err("solo el dueño es capaz de reasignar su id, usted no posee los permisos".to_string())
        }
        /// Dado un AccountId, lo agrega a la lista de editores autorizados 
        /// Posibles result:
        #[ink(message)]
        pub fn autorizar_editor (&mut self, nuevo_editor:AccountId) -> bool{ //TERMINAR. Devolver Result
            if self.es_duenio(){
                self.editores.insert(nuevo_editor.clone(),&nuevo_editor); 
                return true;
            }
            false
        }

        /// Dado un AccountId, lo elimina de la lista de editores autorizados 
        /// Posibles result:
        #[ink(message)]
        pub fn desautorizar_editor (&mut self, editor:AccountId) -> bool{ //TERMINAR. Devolver Result
            if self.es_duenio(){
                self.editores.remove(editor);
                return true;
            }
            false
        }

        /// Dada una cantidad, la setea como nueva cantidad de pagos consecutivos sin atrasos necesarios para descuento
        /// Posibles result:
        #[ink(message)]
        pub fn set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento(&mut self,cant:u32) -> bool{  //TERMINAR. Devolver Result
            if !self.tiene_permiso() {return false;}

            self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento=cant;

            true
        }

        /// Dados un nombre, apellido, dni y una categoria, registra un nuevo socio y le crea una primer cuota a vencer dentro de 10 dias
        /// Posibles result:
        #[ink(message)]
        pub fn registrar_nuevo_socio(&mut self,nombre:String,apellido:String,dni:u32,categoria:Categoria) -> bool{ //TERMINAR. Devolver Result
            if !self.tiene_permiso() {return false;}
            if !self.categorias_data.contains(categoria.discriminant()) {return false;}

            let info_personal_del_socio=DatosPersonalesSocio{nombre,apellido,dni:dni.clone()};
            let socio=Socio{id:self.nueva_id(TipoId::Socio),categoria,datos_personales:info_personal_del_socio};
            self.socios.insert(socio.id, &socio);

            return self.crear_cuota_para_socio(socio.id, FechaTemporalDespuesBorrar {mes:0}
                                            /* datetime::LocalDateTime::now().add_seconds(604800)*/); 
                                        // reemplazar 604800 por -> 10*(constante cantidad_segundos_por_dia)
        }

        /// Crea una cuota a vencer el dia 10 del mes actual para todos los usuarios
        /// Este metodo se puede llamar manualmente, y ademas se llama cada vez que se quiere realizar un pago
        /// Este metodo se ejecutara como mucho una vez por mes. De ser llamado mas veces devolvera un Result
        /// Posibles result:
        #[ink(message)]
        pub fn actualizacion_mensual(&mut self) -> bool{ //TERMINAR. Devolver Result
            if !self.tiene_permiso() {return false;}
            let meses_desde_la_ultima_actualizacion = self.FECHA_DE_HOY_DESPUES_BORRAR.mes - self.fecha_de_la_ultima_actualizacion.mes;
            if !(meses_desde_la_ultima_actualizacion>0) {return false;}

            for i in 1..self.mapping_lens.socios+1 {
                self.crear_cuota_para_socio(i, FechaTemporalDespuesBorrar {mes: self.FECHA_DE_HOY_DESPUES_BORRAR.mes}
                                        /* datetime::LocalDateTime::mes_y_anio_actual_dia_10()*/); 
            }
            self.fecha_de_la_ultima_actualizacion = self.FECHA_DE_HOY_DESPUES_BORRAR.clone();
            true
        }

        /// Dados un dni y un monto, marca como pagado el pago sin pagar mas viejo
        /// Posibles result:
        #[ink(message)]
        pub fn registrar_nuevo_pago(&mut self, dni_socio:u32, monto:u32 ) -> bool{ //TERMINAR. Devolver Result
            self.actualizacion_mensual();

            if !self.tiene_permiso() {return false;}
            let Some(socio) = self.get_socio_dni(dni_socio) else {return false;};
            
            let Some(pago) = self.get_primer_pago_sin_acreditar(socio.id) else {return false;};
            if !(pago.monto == monto as u128) {return false;};

            self.marcar_pago_pagado(pago.id)
        }

        /// Si quien llama es el duenio, activa la politica de autorizacion. Por lo cual solo los AccountId autorizados pueden editar 
        #[ink(message)]
        pub fn activar_politica_de_autorizacion (&mut self) -> bool{ //TERMINAR. Devolver Result
            if self.es_duenio(){
                self.politica_de_autorizacion_activada=true;
                return true;
            }
            false
        }
        /// Si quien llama es el duenio, desactiva la politica de autorizacion. Por lo cual cualquien AccountId puede editar 
        #[ink(message)]
        pub fn desactivar_politica_de_autorizacion (&mut self) -> bool{ //TERMINAR. Devolver Result
            if self.es_duenio(){
                self.politica_de_autorizacion_activada=false;
                return true;
            }
            false
        }




        // ----------- Getters

        /// Devuelve true si quien llama a este metodo es el duenio del Club, false en caso contrario
        #[ink(message)]
        pub fn soy_FINNALBOSS(&self) -> bool{
            self.es_duenio()
        }
        /// Devuelve true si quien llama a este metodo tiene permisos para editar los datos del CLub, false en caso contrario
        #[ink(message)]
        pub fn puedo_editar(&self) -> bool{
            self.tiene_permiso()
        }

        /// Devuelve la cantidad de pagos consecutivos sin atrasos necesarios para descuento
        #[ink(message)]
        pub fn get_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento(&mut self)->u32{
            self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento
        }

        /// Dado un dni se listan sus pagos, mostrando la informacion del socio, la categoria y el monto pagado
        /// Si se ingresa None, de listaran todos los pagos organizados por usuarios
        /// MODIFICAR: No cumplimos con la consigna, tenemos que devolver otro struct (anonimo?), en el que se guarde la información del socio, la categoría y el monto pagado. Actualmente devolvemos el struct Pago, lo cual esta MAL
        #[ink(message)]
        pub fn consulta_de_pago(&self, dni_ingresado:Option<u32>)->Option<Vec<Pago>>{ //TERMINAR. Devolver Result (cuando devuelve None tendria que ser Err, cuando devuelve Some, tendria que ser Ok)
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
        pub fn get_pagos_del_mes(&self,fecha:FechaTemporalDespuesBorrar) -> Vec<Pago>{
            self.pagos.get_or_default().into_iter().filter(|p|p.fecha_de_vencimiento.mes == fecha.mes).collect()
        }
        
        #[ink(message)]
        pub fn get_categoria_data(&self,categoria:Categoria) -> DatosCategoria{ 
            self.categorias_data.get(categoria.discriminant()).unwrap().clone() // CONSULTAR: SI O SI, SI EXISTE LA CATEGORIA TENDRIA Q EXISTIR SU DATA. HACERLO EN EL CONSTRUCTOR? SINO DEVOLVER UN OPTION Y LISTO. TERMINAR
        }

        #[ink(message)]
        pub fn categoria_de(&self,socio_id:u32)->Option<u32>{ // CONSULTAR: ESTO ESTA BIEN? ESTAS ACCEDIENDO A LA PRIVACIDAD DEL SOCIO PARA OBTENER SUS DATOS
            if let Some(socio) = self.socios.get(socio_id){
                return Some(socio.categoria.discriminant())
            }
            None
        } 
*/
        #[ink(message)]
        pub fn cant_categorias(&self) -> u32 {
            //mem::variant_count::<Categoria>() as u32 // use std::mem;
            Categoria::Cantidad.discriminant() as u32
        }


    }
    

    // CONSULTAR: 
        // 1- BOOOOOOOOOOOOOOL PORFAVORRRRRRRRRRRRR => No
        // 2- Fecha? Que hacemos? "the trait bound `LocalDateTime: TypeInfo` is not satisfied" => Probar con date_time

    // TERMINAR:



    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        
    }
}
