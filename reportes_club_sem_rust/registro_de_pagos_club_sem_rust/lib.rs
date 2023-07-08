#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::registro_de_pagos_club_sem_rust::ClubSemRustRef;
#[ink::contract]
mod registro_de_pagos_club_sem_rust {

    use ink::storage::Mapping;
    use ink::prelude::string::String;
    use ink::storage::Lazy;

    use datetime::LocalDateTime;
    use datetime::DatePiece;
    use datetime::LocalDate;
    use datetime::LocalTime;

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
        pub fn discriminant(&self) -> u32 {
            unsafe { *(self as *const Self as *const u32) }
        }
    }
    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq)]
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
        costo_mensual_en_tokens:u32,
        actividades_accesibles_base: Vec<Actividad>,
    }
    impl DatosCategoria{
        pub fn id (&self) -> u32{
            self.id
        }
        pub fn costo_mensual_en_tokens (&self) -> u32{
            self.costo_mensual_en_tokens
        }
        pub fn actividades_accesibles_base (&self) ->  Vec<Actividad>{
            self.actividades_accesibles_base.clone()
        }
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
    impl Pago{
        pub fn socio_id(&self) -> u32{
            self.socio_id
        }
        pub fn id(&self) -> u32{
            self.id
        }
        pub fn fecha_de_pago(&self) -> Option<Timestamp>{
            self.fecha_de_pago.clone()
        }
        pub fn fecha_de_vencimiento(&self) -> Timestamp{
            self.fecha_de_vencimiento.clone()
        }
        pub fn monto(&self) -> u128{
            self.monto
        }
        pub fn tiene_bonificacion(&self) -> bool{
            self.tiene_bonificacion
        }
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
                return fecha_de_pago < pago.fecha_de_vencimiento
            }
            return LocalDateTime::now() < LocalDateTime::at(pago.fecha_de_vencimiento as i64);
        }

        fn crear_cuota_para_socio(&mut self, socio_id:u32, fecha_de_vencimiento:LocalDateTime) -> Result<String,String>{
            if !self.socios.contains(socio_id) {return Err("el socio no esta registrado en el club".to_string())}

            let id_categoria_del_usuario = self.socios.get(socio_id).unwrap().categoria.discriminant();
            if !self.categorias_data.contains(id_categoria_del_usuario) {return Err("no se cargo la data de la categoria".to_string());}

            let mut monto_cuota=self.categorias_data.get::<u32>(id_categoria_del_usuario).unwrap().costo_mensual_en_tokens;
            let cumple_las_condiciones_para_obtener_la_bonificacion = self.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id);
            if cumple_las_condiciones_para_obtener_la_bonificacion {monto_cuota -= monto_cuota*self.porcentaje_de_descuento_por_bonificacion}

            let cuota=Pago{id:self.nueva_id(TipoId::Pago),socio_id,fecha_de_pago:None,
                fecha_de_vencimiento:fecha_de_vencimiento.to_instant().seconds() as u64,
                monto:(monto_cuota as u128), 
                tiene_bonificacion:cumple_las_condiciones_para_obtener_la_bonificacion
            }; 
            
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
        fn marcar_pago_pagado(&mut self, pago_id:u32) -> Result<String,String>{  // calculo que este tambien da para result
            let mut pagos = self.pagos.get_or_default();
            if pagos.len()<pago_id as usize {return Err("el pago nunca se registro".to_string());};

            if let Some(_) = pagos[pago_id as usize].fecha_de_pago.clone(){return Err("el pago ya habia sido pagado con anterioridad".to_string());};

            pagos[pago_id as usize].fecha_de_pago = Some(self.env().block_timestamp());
            self.pagos.set(&pagos);
            
            Ok("la operacion se realizo con exito".to_string())
        }


        // ----------- Setters 

        /// Agrega la data de una categoria. 
        /// Si no se agregan la data de todas las categorias, otros metodos que quieran utilizarla devolveran un Err
        /// Posibles result:
        #[ink(message)]
        pub fn cargar_data_categoria(&mut self, categoria:Categoria, costo_mensual_en_tokens:u32, actividades_accesibles_base: Vec<Actividad>) -> Result<String,String>{ // TERMINAR
            if !self.tiene_permiso() {return Err("el socio no tiene permitido registrar un nuevo socio, puesto que no posee los permisos necesarios".to_string());}
            self.categorias_data.insert(categoria.discriminant(),&DatosCategoria{id:categoria.discriminant(), costo_mensual_en_tokens,actividades_accesibles_base });
            Ok("la operacion se realizo con exito".to_string())
        }

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
        pub fn autorizar_editor (&mut self, nuevo_editor:AccountId) -> Result<String,String>{ //TERMINAR. Devolver Result
            if self.es_duenio(){
                self.editores.insert(nuevo_editor.clone(),&nuevo_editor); 
                return Ok("la operacion se realizo exitosamente".to_string());
            }
            return Err("el socio no tiene permitido autorizar un editor, puesto que no posee los permisos".to_string());
        }

        /// Dado un AccountId, lo elimina de la lista de editores autorizados 
        /// Posibles result:
        #[ink(message)]
        pub fn desautorizar_editor (&mut self, editor:AccountId) -> Result<String,String>{ //TERMINAR. Devolver Result
            if self.es_duenio(){
                self.editores.remove(editor);
                return Ok("la operacion se realizo exitosamente".to_string());
            }
            return Err("el socio no tiene permitido desautorizar un editor, puesto que no posee los permisos necesarios".to_string());
        }

        /// Dada una cantidad, la setea como nueva cantidad de pagos consecutivos sin atrasos necesarios para descuento
        /// Posibles result:
        #[ink(message)]
        pub fn set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento(&mut self,cant:u32) -> Result<String,String>{  //TERMINAR. Devolver Result
            if !self.tiene_permiso() {return Err("el socio no tiene permitido cambiar la cantidad de pagos consecutivos necesarios para activar la bonificacion, puesto que no tiene los permisos necesarios".to_string());}

            self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento=cant;

            return Ok("la operacion se realizo exitosamente".to_string());
        }

        /// Dados un nombre, apellido, dni y una categoria, registra un nuevo socio y le crea una primer cuota a vencer dentro de 10 dias
        /// Posibles result:
        /// TERMINAR
        #[ink(message)]
        pub fn registrar_nuevo_socio(&mut self,nombre:String,apellido:String,dni:u32,categoria:Categoria) -> Result<String,String>{ //TERMINAR. Devolver Result
            if !self.tiene_permiso() {return Err("el socio no tiene permitido registrar un nuevo socio, puesto que no posee los permisos necesarios".to_string());}
            if !self.categorias_data.contains(categoria.discriminant()) {return Err("no se cargo la data de la categoria".to_string());}

            let info_personal_del_socio=DatosPersonalesSocio{nombre,apellido,dni:dni.clone()};
            let socio=Socio{id:self.nueva_id(TipoId::Socio),categoria,datos_personales:info_personal_del_socio};
            self.socios.insert(socio.id, &socio);

            return self.crear_cuota_para_socio(socio.id, LocalDateTime::now().add_seconds(604800)); 
                                        // reemplazar 604800 por -> 10*(constante cantidad_segundos_por_dia)
        }

        fn es_momento_de_otra_actualizacion(&self) -> bool{
            LocalDateTime::now().date().month() == LocalDateTime::at(self.fecha_de_la_ultima_actualizacion as i64).date().month()
        }

        /// Crea una cuota a vencer el dia 10 del mes actual para todos los usuarios
        /// Este metodo se puede llamar manualmente, y ademas se llama cada vez que se quiere realizar un pago
        /// Este metodo se ejecutara como mucho una vez por mes. De ser llamado mas veces devolvera un Result
        /// Posibles result:
        #[ink(message)]
        pub fn actualizacion_mensual(&mut self) ->Result<String,String>{ //TERMINAR. Devolver Result
            if !self.tiene_permiso() {return Err("el socio no tiene los permisos requeridos para realizar la operacion".to_string());}
            if !self.es_momento_de_otra_actualizacion() {return Err("la actualizacion ya se realizo este mes".to_string());}

            let ahora = LocalDateTime::now();
            for i in 1..self.mapping_lens.socios+1 {
                                                                                                                        // reemplazar 604800 por -> 10*(constante cantidad_segundos_por_dia)
                match self.crear_cuota_para_socio(i, LocalDateTime::new(LocalDate::ymd(ahora.date().year(),ahora.date().month(),1).unwrap(),LocalTime::midnight()).add_seconds(604800)){
                    Ok(_)=>{},
                    Err(_)=>{}
                } 
            }
            self.fecha_de_la_ultima_actualizacion = ahora.to_instant().seconds() as u64;
            Ok("la operacion se realizo con exito".to_string())
        }

        /// Dados un dni y un monto, marca como pagado el pago sin pagar mas viejo
        /// Posibles result:
        #[ink(message)]
        pub fn registrar_nuevo_pago(&mut self, dni_socio:u32, monto:u32 ) -> Result<String,String>{ //TERMINAR. Devolver Result
            match self.actualizacion_mensual(){
                Ok(_)=>{},
                Err(_)=>{}
            }
            if !self.tiene_permiso() {return Err("el socio no posee los permisos para realizar esta operacion".to_string());}
            let Some(socio) = self.get_socio_dni(dni_socio) else {return Err("no se encuentra ningun socio registrado con el id proporcionado".to_string());};
            
            let Some(pago) = self.get_primer_pago_sin_acreditar(socio.id) else {return Err("el socio no posee pagos sin acreditar".to_string());};
            if !(pago.monto == monto as u128) {return Err("el monto insertado no corresponde con el del pago a acreditar".to_string());};
            self.marcar_pago_pagado(pago.id)
        }

        /// Si quien llama es el duenio, activa la politica de autorizacion. Por lo cual solo los AccountId autorizados pueden editar 
        #[ink(message)]
        pub fn activar_politica_de_autorizacion (&mut self) -> Result<String,String>{ //TERMINAR. Devolver Result
            if self.es_duenio(){
                self.politica_de_autorizacion_activada=true;
                return Ok("la operacion se realizo con exito".to_string());
            }
            Err("no se posee los permisos necesarios para realizar esta operacion".to_string())
        }
        /// Si quien llama es el duenio, desactiva la politica de autorizacion. Por lo cual cualquien AccountId puede editar 
        #[ink(message)]
        pub fn desactivar_politica_de_autorizacion (&mut self) -> Result<String,String>{ //TERMINAR. Devolver Result
            if self.es_duenio(){
                self.politica_de_autorizacion_activada=false;
                return Ok("la operacion se realizo con exito".to_string());
            }
            Err("no se posee los permisos necesarios para realizar esta operacion".to_string())
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
        pub fn consulta_de_pago(&self, dni_ingresado:Option<u32>)->Result<Vec<Pago>,String>{ //TERMINAR. Devolver Result (cuando devuelve None tendria que ser Err, cuando devuelve Some, tendria que ser Ok)
            if !self.tiene_permiso() {return Err("no se tiene los permisos necesarios para realizar esta operaacion".to_string());}
            let Some(dni_ingresado) = dni_ingresado else {return Ok(self.SLICE_O_PAGINACION_QUE_ES_ESO(self.pagos.get_or_default())); }; //CONSULTA: tambien podria ser self.pagos.get();
            let Some(socio) = self.get_socio_dni(dni_ingresado) else {return Err("el dni ingresado no corresponde a ningun socio del club".to_string());};
            let pagos_de_un_socio=self.listar_pagos(socio.id);
            Ok(self.SLICE_O_PAGINACION_QUE_ES_ESO(pagos_de_un_socio))
        }
        #[ink(message)]
        pub fn get_pagos(&self) -> Vec<Pago>{
            self.pagos.get_or_default()
        }
        #[ink(message)]
        pub fn get_pagos_del_mes(&self,fecha:Timestamp) -> Vec<Pago>{
            let fecha = LocalDateTime::at(fecha as i64);
            self.pagos.get_or_default().into_iter().filter(|p|LocalDateTime::at(p.fecha_de_vencimiento as i64).date().month() == fecha.date().month() && LocalDateTime::at(p.fecha_de_vencimiento as i64).date().year() == fecha.date().year()).collect()
        }
        
        #[ink(message)]
        pub fn get_categoria_data(&self,categoria:Categoria) -> Result<DatosCategoria,String>{
            if !self.tiene_permiso() {return Err("el socio no tiene permitido registrar un nuevo socio, puesto que no posee los permisos necesarios".to_string());}
            if !self.categorias_data.contains(categoria.discriminant()) {return Err("no se cargo la data de la categoria".to_string());}
            
            Ok(self.categorias_data.get(categoria.discriminant()).unwrap().clone())
        }

        /// Dado un socio ID, retorna su categoria
        /// Ya que Socio es privado, en vez de un getter implementado en Socio, lo hacemos de esta manera
        /// CONSULTAR: ESTO ESTA BIEN? ESTAS ACCEDIENDO A LA PRIVACIDAD DEL SOCIO PARA OBTENER SUS DATOS (Lo usamos en el contrato 2)
        /// Posibles Result: 
        #[ink(message)]
        pub fn categoria_de(&self,socio_id:u32)->Result<Categoria,String>{ 
            let Some(socio) = self.socios.get(socio_id) else {return Err("el socio_id ingresado no es correcto".to_string());};
            
            Ok(socio.categoria)
        } 

        // CONSULTAR. ARREGLAR. TERMINAR. MODIFICAR. PEDIR DISCULPAS
        #[ink(message)]
        pub fn cant_categorias(&self) -> u32 {
            //mem::variant_count::<Categoria>() as u32 // use std::mem;
            Categoria::Cantidad.discriminant() as u32
        }

        
        #[ink(message)]
        pub fn socio_tiene_permitida_la_asistencia_a(&self, socio_id:u32,id_actividad: u32) -> Result<bool,String>{
            let actividad = Actividad::Gimnasio; // pasar de id_actividad a actividad TEMRINAR ARRAGLAR
            let Ok(categoria) = self.categoria_de(socio_id) else {return Err("el socio_id ingresado no es correcto".to_string())}; //MODIFICAR. Hacer que solamente devuelva el que venga
            match categoria{
                Categoria::B{id_deporte_seleccionado_por_el_usuario} => {if id_deporte_seleccionado_por_el_usuario == actividad as u32 {return Ok(true);}}
                _ => {}
            }

            //let res = self.get_categoria_data(categoria).unwrap().actividades_accesibles_base().iter().find(|&&a|a == actividad); // MODIFICAR. Sacar el unwrap y manejarlo
            //Ok(res.is_some())
            Err("aa".to_string())
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
        #[test]
        fn test_creacion_club_sem_rust(){
            let duenio=AccountId::from([0x1;32]);
            ink::env::test::set_caller::<ink::env::DefaultEnvaironment>(duenio);
            let club= ClubSemRust::new(duenio,3,10);
        }
    }
}
