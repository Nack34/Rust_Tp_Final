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

    /// Los posibles tipos de errores all llamar a los metodos del contrato
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Devuelto si el socio buscado no se encuentra registrado en el club. 
        SocioNoRegistrado,
        /// Devuelto si el Id de categoria ingresado no representa una categoria del Club.
        ActividadInvalida,
        /// Devuelto si el Id de actividad ingresado no representa una actividad del Club.
        CategoriaInvalida,
        /// Devuelto si no se cargo la data de la categoria buscada.
        CategoriaSinData,
        /// Devuelto si el pago buscado nunca se registro.
        PagoNoRegistrado,
        /// Devuelto si el pago que se quiso pagar ya habia sido pagado con anterioridad.
        PagoYaPagado,
        /// Devuelto si el AccountId que llama no tiene permitido registrar un nuevo socio, puesto que no posee los permisos necesarios.
        NoSePoseenLosPermisosParaEditar,
        /// Devuelto si no transcurrio el tiempo necesario para que este metodo vuelva a ejecutarse
        NoTranscurrioElTiempoNecesarioDesdeElUltimoLlamado,
        /// Devuelto si se quiere pagar un pago de un socio que no posee pagos sin acreditar
        SocioNoPoseePagosSinAcreditar,
        /// Devuelto si el monto ingresado no corresponde con el del pago a acreditar
        MontoInvalido,        
    }

    /// Voy a llorar del asco que me da hacer "Cantidad". Pido disculpas por los malestares emocionales que esto pueda llegar a crear
    /// "Cantidad"  puede dar errores
    /// MODIFICAR. TERMINAR. PEDIR DISCULPAS 
    #[derive(scale::Decode, scale::Encode,Debug, Clone)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Categoria{ 
        A,
        B{deporte_seleccionado_por_el_usuario:Actividad},
        C,
        Cantidad, 
    }
    impl Categoria {
        pub fn discriminant(&self) -> u32 {
            unsafe { *(self as *const Self as *const u32) }
        }
    }
    
    /// Voy a llorar del asco que me da hacer "Cantidad". Pido disculpas por los malestares emocionales que esto pueda llegar a crear
    /// "Cantidad"  puede dar errores
    /// MODIFICAR. TERMINAR. PEDIR DISCULPAS 
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
        Paddle,
        Cantidad, 
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
    #[derive(scale::Decode, scale::Encode,Debug,Clone)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct DatosCategoria{
        /// El id de la categoria.
        /// Se obtiene al cnvertir el enum en u32
        id:u32, 
        /// El costo mensual en tokens de la catagoria.
        costo_mensual_en_tokens:u32,
        /// Las actividades accesibles base.
        actividades_accesibles_base: Vec<Actividad>,
    }
    impl DatosCategoria{
        /// Retorna el id de la categoria de los cuales estos datos son
        pub fn id (&self) -> u32{
            self.id
        }
        /// Retorna el costo mensual (en tokens) de la categoria de los cuales estos datos son
        pub fn costo_mensual_en_tokens (&self) -> u32{
            self.costo_mensual_en_tokens
        }
        /// Retorna las actividades base que la categoria, de los cuales estos datos son, tiene acceso
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
        /// Retorna el id del socio que realizo el pago
        pub fn socio_id(&self) -> u32{
            self.socio_id
        }
        /// Retorna el id del pago
        pub fn id(&self) -> u32{
            self.id
        }
        /// Retorna la fecha en la que se realizo el pago del pago, si se realizo
        pub fn fecha_de_pago(&self) -> Option<Timestamp>{
            self.fecha_de_pago.clone()
        }
        /// Retorna la fecha de vencimiento del pago
        pub fn fecha_de_vencimiento(&self) -> Timestamp{
            self.fecha_de_vencimiento.clone()
        }
        /// Retorna el monto del pago
        pub fn monto(&self) -> u128{
            self.monto
        }
        /// Retorna true si el pago fue con bonificacion, false en caso contrario
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
        /// Se podrá activar o desactivar esta política de autorización por parte del dueño del contrato. 
        /// Si está desactivada, cualquiera podrá realizar operaciones; si está activada, sólo las direcciones autorizadas podrán hacerlo
        politica_de_autorizacion_activada:bool,

        /// El duenio del contrato.
        /// 
        /// Tendrá el poder de siempre tener permiso para cualquier operacion. 
        /// Podra autorizar o desautorizar a los editores
        /// Podra activar o desactivar la politica de actorizacion 
        duenio_account_id:AccountId,

        /// Los editores del contrato.
        /// 
        /// Son los que tendran el permiso de realizar operaciones de ser necesaria una autorizacion
        editores:Mapping<AccountId,AccountId>,

        /// Los socios del Club.
        socios:Mapping<u32,Socio>,

        /// Las datas de las categorias de los socios.
        /// 
        /// Es necesario cargar las datas de las categorias para que el club funcione correctamente.
        /// Pueden modificarse cuando se requiera
        categorias_data:Mapping<u32,DatosCategoria>,

        /// Todos los pagos realizados
        pagos:Lazy<Vec<Pago>>, // CONSULTAR: Lazy

        /// MappingLens es una estructura auxiliar creada para, de ser necesario, guardar los tamanios de los mappings de ClubSemRust
        mapping_lens:MappingLens,

        /// La cantidad de pagos consecutivos sin atrasos necesarios para descuento en las cuotas de los usuarios
        cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento:u32,

        /// El porcentaje de descuento que se les hara a las cuotas de los usuarios si tienen bonificacion 
        porcentaje_de_descuento_por_bonificacion:u32,

        /// Fecha de la ultima actualizacion. 
        /// Es necesario hacer la actualizacion una vez por mes
        fecha_de_la_ultima_actualizacion:Timestamp
    }

    impl ClubSemRust {
        #[ink(constructor)]
        pub fn new(duenio_account_id:AccountId, cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento:u32, porcentaje_de_descuento_por_bonificacion:u32,politica_de_autorizacion_activada:bool)->Self{
            ClubSemRust::new_priv(duenio_account_id, cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento, porcentaje_de_descuento_por_bonificacion,politica_de_autorizacion_activada)
        }
        fn new_priv(duenio_account_id:AccountId, cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento:u32,mut porcentaje_de_descuento_por_bonificacion:u32,politica_de_autorizacion_activada:bool)->Self{
            if porcentaje_de_descuento_por_bonificacion > 99 {porcentaje_de_descuento_por_bonificacion = 0} 
            
            let mut csr = Self{
                politica_de_autorizacion_activada,
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

        /// Retorna una nueva Id para el TipoId seleccionado. 
        /// Siempre incrementa
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

        /// Crea una cuota a vencer en fecha_de_vencimiento para el socio solicitado
        /// 
        /// Posibles Error: SocioNoRegistrado, CategoriaSinData
        fn crear_cuota_para_socio(&mut self, socio_id:u32, fecha_de_vencimiento:LocalDateTime) -> Result<(),Error>{
            if !self.socios.contains(socio_id) {return Err(Error::SocioNoRegistrado)}

            let id_categoria_del_usuario = self.socios.get(socio_id).unwrap().categoria.discriminant();
            if !self.categorias_data.contains(id_categoria_del_usuario) {return Err(Error::CategoriaSinData);}

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
            Ok(())
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
        fn marcar_pago_pagado(&mut self, pago_id:u32) -> Result<(),Error>{  // calculo que este tambien da para result
            let mut pagos = self.pagos.get_or_default();
            if pagos.len()<pago_id as usize {return Err(Error::PagoNoRegistrado);};

            if let Some(_) = pagos[pago_id as usize].fecha_de_pago.clone(){return Err(Error::PagoYaPagado);};

            pagos[pago_id as usize].fecha_de_pago = Some(self.env().block_timestamp());
            self.pagos.set(&pagos);
            
            Ok(())
        }


        // ----------- Setters 

        /// Agrega la data de una categoria. 
        /// Si ya habia data de esa categoria se pisara con la nueva ingresada.
        /// 
        /// Si no se agregan la data de todas las categorias, otros metodos que quieran utilizarla devolveran un Err
        /// 
        /// Se necesitan permisos para ejecutar
        /// 
        /// Posibles Error: NoSePoseenLosPermisosParaEditar
        #[ink(message)]
        pub fn cargar_data_categoria(&mut self, categoria:Categoria, costo_mensual_en_tokens:u32, actividades_accesibles_base: Vec<Actividad>) -> Result<(),Error>{ 
            self.cargar_data_categoria_priv(categoria,costo_mensual_en_tokens,actividades_accesibles_base)
        }
        fn cargar_data_categoria_priv(&mut self, categoria:Categoria, costo_mensual_en_tokens:u32, actividades_accesibles_base: Vec<Actividad>) -> Result<(),Error>{// TERMINAR
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosParaEditar);}
            self.categorias_data.insert(categoria.discriminant(),&DatosCategoria{id:categoria.discriminant(), costo_mensual_en_tokens,actividades_accesibles_base });
            Ok(())
        }

        /// Dado un AccountId, lo setea como nuevo duenio del Club
        /// 
        /// Solo puede ejecutarse si quien llama a este metodo es el duenio del Club
        /// 
        /// Posibles Error: NoSePoseenLosPermisosParaEditar
        #[ink(message)]
        pub fn set_duenio (&mut self, nuevo_duenio_account_id:AccountId) -> Result<(),Error>{
            self.set_duenio_priv(nuevo_duenio_account_id)
        }
        fn set_duenio_priv(&mut self, nuevo_duenio_account_id:AccountId) -> Result<(),Error>{
            if !self.es_duenio(){ return Err(Error::NoSePoseenLosPermisosParaEditar)}
        
            self.duenio_account_id = nuevo_duenio_account_id; 
            return Ok(());
        }
        /// Dado un AccountId, lo agrega a la lista de editores autorizados 
        /// 
        /// Solo puede ejecutarse si quien llama a este metodo es el duenio del Club
        /// 
        /// Posibles Error: NoSePoseenLosPermisosParaEditar
        #[ink(message)]
        pub fn autorizar_editor (&mut self, nuevo_editor:AccountId) -> Result<(),Error>{ 
            self.autorizar_editor_priv(nuevo_editor)
        }
        fn autorizar_editor_priv (&mut self, nuevo_editor:AccountId) -> Result<(),Error>{ 
            if !self.es_duenio(){ return Err(Error::NoSePoseenLosPermisosParaEditar)} 
            
            self.editores.insert(nuevo_editor.clone(),&nuevo_editor); 
            return Ok(());
        }

        /// Dado un AccountId, lo elimina de la lista de editores autorizados 
        /// 
        /// Solo puede ejecutarse si quien llama a este metodo es el duenio del Club
        /// 
        /// Posibles Error: NoSePoseenLosPermisosParaEditar
        #[ink(message)]
        pub fn desautorizar_editor (&mut self, editor:AccountId) -> Result<(),Error>{ 
            self.desautorizar_editor_priv(editor)
        }
        fn desautorizar_editor_priv (&mut self, editor:AccountId) -> Result<(),Error>{
            if !self.es_duenio(){ return Err(Error::NoSePoseenLosPermisosParaEditar)}
            
            self.editores.remove(editor);
            return Ok(());
        }

        /// Dada una cantidad, la setea como nueva cantidad de pagos consecutivos sin atrasos necesarios para descuento
        /// 
        /// Se necesitan permisos para ejecutar
        /// 
        /// Posibles result: NoSePoseenLosPermisosParaEditar
        #[ink(message)]
        pub fn set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento(&mut self,cant:u32) -> Result<(),Error>{  
            self.set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento_priv(cant)
        }
        fn set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento_priv(&mut self,cant:u32) -> Result<(),Error>{ 
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosParaEditar);}

            self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento=cant;

            return Ok(());
        }

        /// Dados un nombre, apellido, dni y una categoria, registra un nuevo socio y le crea una primer cuota a vencer dentro de 10 dias
        /// 
        /// Se necesitan permisos para ejecutar
        /// 
        /// Posibles result: NoSePoseenLosPermisosParaEditar, CategoriaSinData, SocioNoRegistrado
        #[ink(message)]
        pub fn registrar_nuevo_socio(&mut self,nombre:String,apellido:String,dni:u32,categoria:Categoria) -> Result<(),Error>{
            self.registrar_nuevo_socio_priv(nombre, apellido, dni, categoria)
        }
        fn registrar_nuevo_socio_priv(&mut self,nombre:String,apellido:String,dni:u32,categoria:Categoria) -> Result<(),Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosParaEditar);}
            if !self.categorias_data.contains(categoria.discriminant()) {return Err(Error::CategoriaSinData);}

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
        pub fn actualizacion_mensual(&mut self) ->Result<(),Error>{ //TERMINAR. Devolver Result
            self.actualizacion_mensual_priv()
        }
        fn actualizacion_mensual_priv(&mut self) ->Result<(),Error>{ //TERMINAR. Devolver Result
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosParaEditar);}
            if !self.es_momento_de_otra_actualizacion() {return Err(Error::NoTranscurrioElTiempoNecesarioDesdeElUltimoLlamado);}

            let ahora = LocalDateTime::now();
            for i in 1..self.mapping_lens.socios+1 {
                                                                                                                        // reemplazar 604800 por -> 10*(constante cantidad_segundos_por_dia)
                match self.crear_cuota_para_socio(i, LocalDateTime::new(LocalDate::ymd(ahora.date().year(),ahora.date().month(),1).unwrap(),LocalTime::midnight()).add_seconds(604800)){
                    Ok(_)=>{},
                    Err(_)=>{}
                } 
            }
            self.fecha_de_la_ultima_actualizacion = ahora.to_instant().seconds() as u64;
            Ok(())
        }

        /// Dados un dni y un monto, marca como pagado el pago sin pagar mas viejo
        /// Posibles result:
        #[ink(message)]
        pub fn registrar_nuevo_pago(&mut self, dni_socio:u32, monto:u32 ) -> Result<(),Error>{ //TERMINAR. Devolver Result
            self.registrar_nuevo_pago_priv(dni_socio,monto)
        }
        fn registrar_nuevo_pago_priv(&mut self, dni_socio:u32, monto:u32 ) -> Result<(),Error>{ //TERMINAR. Devolver Result
            match self.actualizacion_mensual(){
                Ok(_)=>{},
                Err(_)=>{}
            }
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosParaEditar);}
            let Some(socio) = self.get_socio_dni(dni_socio) else {return Err(Error::SocioNoRegistrado);};
            
            let Some(pago) = self.get_primer_pago_sin_acreditar(socio.id) else {return Err(Error::SocioNoPoseePagosSinAcreditar);};
            if !(pago.monto == monto as u128) {return Err(Error::MontoInvalido);};
            self.marcar_pago_pagado(pago.id)
        }

        /// Activa la politica de autorizacion. 
        /// Solo puede ejecutarse si quien llama a este metodo es el duenio del Club
        #[ink(message)]
        pub fn activar_politica_de_autorizacion (&mut self) -> Result<(),Error>{ //TERMINAR. Devolver Result
            self.activar_politica_de_autorizacion_priv()
        }
        fn activar_politica_de_autorizacion_priv (&mut self) -> Result<(),Error>{ //TERMINAR. Devolver Result
            if !self.es_duenio(){return Err(Error::NoSePoseenLosPermisosParaEditar)}
            
            self.politica_de_autorizacion_activada=true;
            Ok(())
        }
        
        /// Desactiva la politica de autorizacion. 
        /// Solo puede ejecutarse si quien llama a este metodo es el duenio del Club
        #[ink(message)]
        pub fn desactivar_politica_de_autorizacion (&mut self) -> Result<(),Error>{ //TERMINAR. Devolver Result
            self.desactivar_politica_de_autorizacion_priv()
        }
        fn desactivar_politica_de_autorizacion_priv (&mut self) -> Result<(),Error>{ //TERMINAR. Devolver Result
            if !self.es_duenio(){return Err(Error::NoSePoseenLosPermisosParaEditar)}
            
            self.politica_de_autorizacion_activada=false;
            Ok(())
        }




        // ----------- Getters

        /// Retorna todos los ids de actividades posibles del Club
        #[ink(message)]
        pub fn get_ids_categorias(&self) -> Vec<u32>{
            (0..self.cant_categorias()).collect()
        }
        /// Retorna todos los ids de categorias posibles del Club
        #[ink(message)]
        pub fn get_ids_actividades(&self) -> Vec<u32>{
            (0..self.cant_actividades()).collect()
        }

        /// CONSULTAR. ARREGLAR. TERMINAR. MODIFICAR. PEDIR DISCULPAS
        /// 
        /// Retorna la cantidad de categorias posibles de los socios del Club
        #[ink(message)]
        pub fn cant_categorias(&self) -> u32 {
            self.cant_categorias_priv()
        }
        fn cant_categorias_priv(&self) -> u32 {
            //mem::variant_count::<Categoria>() as u32 // use std::mem;
            Categoria::Cantidad.discriminant() -1
        }

        /// CONSULTAR. ARREGLAR. TERMINAR. MODIFICAR. PEDIR DISCULPAS
        /// 
        /// Retorna la cantidad de actividades posibles del Club
        #[ink(message)]
        pub fn cant_actividades(&self) -> u32 {
            self.cant_actividades_priv()
        }
        fn cant_actividades_priv(&self) -> u32 {
            //mem::variant_count::<Actividad>() as u32 // use std::mem;
            Actividad::Cantidad as u32 -1
        }

        /// Devuelve true si quien llama a este metodo es el duenio del Club, false en caso contrario
        #[ink(message)]
        pub fn soy_duenio(&self) -> bool{
            self.soy_duenio_priv()
        }
        fn soy_duenio_priv(&self) -> bool{
            self.es_duenio()
        }
        /// Devuelve true si quien llama a este metodo tiene permisos para editar los datos del CLub, false en caso contrario
        #[ink(message)]
        pub fn puedo_editar(&self) -> bool{
            self.puedo_editar_priv()
        }
        fn puedo_editar_priv(&self) -> bool{
            self.tiene_permiso()
        }
        
        /// Retorna true si el socio con el dni pasado por parametro es un socio del club, false en caso contrario 
        #[ink(message)]
        pub fn existe_socio_dni(&self,un_dni:u32)->bool{
            self.existe_socio_dni_priv(un_dni)
        }
        fn existe_socio_dni_priv(&self,un_dni:u32)->bool{
            self.get_socio_dni(un_dni).is_some()
        }
        
        #[ink(message)]
        pub fn existe_actividad_id (&self, id_actividad:u32) -> bool{
            self.get_ids_actividades().contains(&id_actividad)
        }
        #[ink(message)]
        pub fn existe_categoria_id (&self, id_categoria:u32) -> bool{
            self.get_ids_categorias().contains(&id_categoria)
        }
        
        /// Devuelve la cantidad de pagos consecutivos sin atrasos necesarios para descuento
        #[ink(message)]
        pub fn get_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento(&mut self)->u32{
            self.get_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento_priv()
        }
        fn get_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento_priv(&mut self)->u32{
            self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento
        }

        /// Dado un dni se listan sus pagos, mostrando la informacion del socio, la categoria y el monto pagado
        /// Si se ingresa None, de listaran todos los pagos organizados por usuarios
        /// MODIFICAR: No cumplimos con la consigna, tenemos que devolver otro struct (anonimo?), en el que se guarde la información del socio, la categoría y el monto pagado. Actualmente devolvemos el struct Pago, lo cual esta MAL
        #[ink(message)]
        pub fn consulta_de_pago(&self, dni_ingresado:Option<u32>)-> Result<Vec<Pago>,Error>{ 
            self.consulta_de_pago_priv(dni_ingresado)
        }
        fn consulta_de_pago_priv(&self, dni_ingresado:Option<u32>)->Result<Vec<Pago>,Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosParaEditar)}
            let Some(dni_ingresado) = dni_ingresado else {return Ok(self.SLICE_O_PAGINACION_QUE_ES_ESO(self.pagos.get_or_default())); }; //CONSULTA: tambien podria ser self.pagos.get();
            let Some(socio) = self.get_socio_dni(dni_ingresado) else {return Err(Error::SocioNoRegistrado)};
            let pagos_de_un_socio=self.listar_pagos(socio.id);
            Ok(self.SLICE_O_PAGINACION_QUE_ES_ESO(pagos_de_un_socio))
        }

        /// Retorna todos los pagos que se han realizado al Club
        #[ink(message)]
        pub fn get_pagos(&self) -> Vec<Pago>{
            self.get_pagos_priv()
        }
        fn get_pagos_priv(&self) -> Vec<Pago>{
            self.pagos.get_or_default()
        }

        /// Retorna una lista de los pagos del mes y anio indicados
        #[ink(message)]
        pub fn get_pagos_del_mes_y_anio(&self,fecha:Timestamp) -> Vec<Pago>{
            self.get_pagos_del_mes_y_anio_priv(fecha)
        }
        fn get_pagos_del_mes_y_anio_priv(&self,fecha:Timestamp) -> Vec<Pago>{
            let fecha = LocalDateTime::at(fecha as i64);
            self.pagos.get_or_default().into_iter().filter(|p|LocalDateTime::at(p.fecha_de_vencimiento as i64).date().month() == fecha.date().month() && LocalDateTime::at(p.fecha_de_vencimiento as i64).date().year() == fecha.date().year()).collect()
        }
        
        /// Retorna la data de la categoria pasada por parametro
        /// 
        /// Posibles Error: CategoriaInvalida, CategoriaSinData
        #[ink(message)]
        pub fn get_categoria_data(&self,id_categoria: u32) -> Result<DatosCategoria,Error>{
            self.get_categoria_data_priv(id_categoria)
        }
        fn get_categoria_data_priv(&self,id_categoria:u32) -> Result<DatosCategoria,Error>{
            if !self.existe_categoria_id(id_categoria) {return Err(Error::CategoriaInvalida)}
            if !self.categorias_data.contains(id_categoria) {return Err(Error::CategoriaSinData)}
            
            Ok(self.categorias_data.get(id_categoria).unwrap().clone())
        }

        /// Dado un socio ID, retorna su categoria
        /// 
        /// Ya que Socio es privado, en vez de un getter implementado en Socio, lo hacemos de esta manera
        /// CONSULTAR: ESTO ESTA BIEN? ESTAS ACCEDIENDO A LA PRIVACIDAD DEL SOCIO PARA OBTENER SUS DATOS (Lo usamos en el contrato 2)
        /// 
        /// Posibles Error: SocioNoRegistrado 
        #[ink(message)]
        pub fn categoria_de(&self,socio_id:u32)->Result<Categoria,Error>{ 
            self.categoria_de_priv(socio_id)
        } 
        fn categoria_de_priv(&self,socio_id:u32)->Result<Categoria,Error>{ 
            let Some(socio) = self.socios.get(socio_id) else {return Err(Error::SocioNoRegistrado)};
            
            Ok(socio.categoria)
        } 

        #[ink(message)]
        pub fn todas_las_categorias_tienen_sus_datas_cargadas(&self) ->bool{
            let mut res = true;
            self.get_ids_categorias().into_iter().for_each(|c| res = res && self.get_categoria_data(c).is_ok() );
            res
        }



        /// TERMINAR. MODIFICAR
        /// Retorna true si el socio tiene permitida la asistencia a la actividad, false en caso contrario
        /// 
        /// Posibles Error: ActividadInvalida, SocioNoRegistrado, CategoriaSinData
        #[ink(message)]
        pub fn socio_tiene_permitida_la_asistencia_a(&self, socio_id:u32,id_actividad: u32) -> Result<bool,Error>{
            self.socio_tiene_permitida_la_asistencia_a_priv(socio_id,id_actividad)
        }
        fn socio_tiene_permitida_la_asistencia_a_priv(&self, socio_id:u32,id_actividad: u32) -> Result<bool,Error>{
            if !self.existe_actividad_id(id_actividad) {return Err(Error::ActividadInvalida)}
            
            let binding = self.categoria_de(socio_id);
            let Ok(categoria) = binding else {return Err(binding.err().unwrap())};
            
            match categoria.clone(){
                Categoria::B{deporte_seleccionado_por_el_usuario} => {if deporte_seleccionado_por_el_usuario as u32 == id_actividad {return Ok(true);}}
                _ => {}
            }

            let binding = self.get_categoria_data(categoria.discriminant());
            let Ok(categoria_data) = binding else {return Err(binding.err().unwrap())};

            let binding = categoria_data.actividades_accesibles_base();
            let res = binding.iter().find(|a|a.clone().clone() as u32 == id_actividad);

            Ok(res.is_some())
        }
        

    }
    

    // CONSULTAR: 
        // 1-

    #[cfg(test)]
    mod tests {
        use crate::registro_de_pagos_club_sem_rust::AccountId;
        use crate::registro_de_pagos_club_sem_rust::ClubSemRust;
        use crate::registro_de_pagos_club_sem_rust::Actividad;
        use crate::registro_de_pagos_club_sem_rust::Categoria;

        //--------------------------modulos para evitar repeticion en los test----------------------
        
        
        /// Crea el club con valores cualquiera
        /// 
        /// Tomamos a Alice como la duenia del Club
        fn inicializar_y_crear_club_sem_rust()->ClubSemRust{
            //Set the contract as callee 
            let contract = ink::env::account_id::<ink::env::DefaultEnvironment>();
            ink::env::test::set_callee::<ink::env::DefaultEnvironment>(contract);
            //Set the alice as caller 
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let duenio = accounts.alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(duenio);

            let mut club= ClubSemRust::new(duenio,3,10,true);
            club
        }
        
        /// Carga la data de las categorias
        /// 
        /// Tomamos a Alice como la duenia del Club
        fn crear_y_cargar_categorias(club:&mut ClubSemRust){
            //Set the alice as caller 
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let duenio = accounts.alice;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(duenio);

            let mut vec_de_actividades_a=Vec::new();
            vec_de_actividades_a.push(Actividad::Natacion);
            vec_de_actividades_a.push(Actividad::Futbol);
            vec_de_actividades_a.push(Actividad::Gimnasio);
            vec_de_actividades_a.push(Actividad::Basquet);
            vec_de_actividades_a.push(Actividad::Rugby);
            vec_de_actividades_a.push(Actividad::Hockey);
            vec_de_actividades_a.push(Actividad::Tenis);
            vec_de_actividades_a.push(Actividad::Paddle);
            assert!(club.cargar_data_categoria(Categoria::A, 5000, vec_de_actividades_a).is_ok());

            let mut vec_de_actividades_b=Vec::new();
            vec_de_actividades_b.push(Actividad::Gimnasio);
            // la actividad puede ser cualquiera, no se va a guargar, es solo para obtener el indice
            assert!(club.cargar_data_categoria(Categoria::B { deporte_seleccionado_por_el_usuario:Actividad::Natacion }, 3000, vec_de_actividades_b).is_ok());

            let mut vec_de_actividades_c=Vec::new();
            vec_de_actividades_c.push(Actividad::Gimnasio);
            assert!(club.cargar_data_categoria(Categoria::C,2000,vec_de_actividades_c).is_ok());


            let vec_de_actividades_a = club.get_categoria_data((Categoria::A).discriminant());
            assert!(vec_de_actividades_a.is_ok());
            assert_eq!(vec_de_actividades_a.unwrap().actividades_accesibles_base,vec_de_actividades_a);
            // id
            let categoria_data = club.get_categoria_data(Categoria::A.discriminant());
            assert_eq!(Categoria::A.discriminant(),categoria_data.id());
            // tokens
            assert_eq!(Categoria::A.discriminant(),5000);
            // actividades_disponibles
            let vec_de_actividades_c = club.get_categoria_data((Categoria::C).discriminant());
            assert!(vec_de_actividades_c.is_ok());
            assert_eq!(vec_de_actividades_c.unwrap().actividades_accesibles_base,vec_de_actividades_c);

        }
        
        #[ink::test]
        fn test_creacion_club_sem_rust(){
            let club = inicializar_y_crear_club_sem_rust();
            // Transfer event triggered during initial construction.
            let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());
        }
        #[ink::test]
        fn test_soy_duenio_siendo_duenio(){
            let club = inicializar_y_crear_club_sem_rust();
            assert!(club.soy_duenio());
        }
        #[ink::test]
        fn test_autorizar_editor_y_soy_duenio_siendo_staff(){
            let mut club = inicializar_y_crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let staff=accounts.bob;
            assert!(club.autorizar_editor(staff).is_ok());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);
            assert!(!club.soy_duenio());
            let staff=accounts.charlie;
            assert!(club.autorizar_editor(staff).is_err());
        }
        #[ink::test]
        fn test_soy_duenio_siendo_cliente(){
            let club = inicializar_y_crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let cliente=accounts.charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(cliente);
            assert!(!club.soy_duenio());
        }
        #[ink::test]
        fn test_puedo_editar_siendo_duenio(){
            let club = inicializar_y_crear_club_sem_rust();
            assert!(club.puedo_editar());
        }
        #[ink::test]
        fn test_puedo_editar_siendo_staff(){
            let mut club = inicializar_y_crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let staff=accounts.bob;
            assert!(club.autorizar_editor(staff).is_ok());
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);
            assert!(club.puedo_editar());
        }
        #[ink::test]
        fn test_puedo_editar_siendo_cliente_con_politica_de_autorizacion_activada(){
            let mut club = inicializar_y_crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert(club.desactivar_politica_de_autorizacion().is_ok());
            assert(club.activar_politica_de_autorizacion().is_ok());

            let staff=accounts.bob;
            assert!(club.autorizar_editor(staff).is_ok());
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);
            assert!(club.puedo_editar());

            let cliente=accounts.charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(cliente);
            assert!(!club.puedo_editar());
        }
        #[ink::test]
        fn test_puedo_editar_siendo_cliente_con_politica_de_autorizacion_dasactivada(){
            let mut club = inicializar_y_crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            assert(club.desactivar_politica_de_autorizacion().is_ok());

            let staff=accounts.bob;
            assert!(club.autorizar_editor(staff).is_ok());
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);
            assert!(club.puedo_editar());

            let cliente=accounts.charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(cliente);
            assert!(club.puedo_editar());
        }
        #[ink::test]
        fn test_cantidad_de_pagos_necesarios_para_bonificacion(){
            let mut club = inicializar_y_crear_club_sem_rust();
            assert_eq!(club.get_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento(),3);
        }
        #[ink::test]
        fn test_cargar_data_categoria_y_get_data_categoria(){
            let mut club = inicializar_y_crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            crear_y_cargar_categorias(&mut club);
        }
        #[ink::test]
        fn test_registrar_nuevo_socio(){
            let mut club = inicializar_y_crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            crear_y_cargar_categorias(&mut club);

            assert!(!club.existe_socio(1));
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),1, Categoria::B { deporte_seleccionado_por_el_usuario: (Actividad::Futbol) }).is_ok());
            assert!(club.existe_socio(1));
        }
        #[ink::test]
        fn test_autorizar_editor_asignando_el_editor(){
            let mut club = inicializar_y_crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let staff=accounts.bob;
            club.autorizar_editor(staff);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);
            assert!(club.es_editor())
        }
        #[ink::test]
        fn test_autorizar_editor_sin_asignar_el_editor(){
            let mut club = inicializar_y_crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let staff=accounts.bob;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);
            assert!(!club.es_editor())
        }
        #[ink::test]
        fn test_desautorizar_editor(){
            let mut club = inicializar_y_crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let staff=accounts.bob;
            club.autorizar_editor(staff);
            club.desautorizar_editor(staff);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);
            assert!(!club.es_editor())
        }
        #[ink::test]
        fn set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento(){
            let mut club = inicializar_y_crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            club.set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento(5);
            assert_eq!(club.get_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento(),5);
        }
    }
}
