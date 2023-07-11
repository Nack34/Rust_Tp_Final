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

    use strum::{EnumCount};
    use strum_macros::{EnumCount as EnumCountMacro,FromRepr};
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;

    /// Los posibles tipos de errores all llamar a los metodos del contrato
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode,Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Devuelto si el socio buscado no se encuentra registrado en el club. 
        SocioNoRegistrado,
        /// Devuelto si el Id de actividad ingresado no representa una actividad del Club.
        ActividadInvalida,
        /// Devuelto si el Id de categoria ingresado no representa una categoria del Club.
        CategoriaInvalida,
        /// Devuelto si no se cargo la data de la categoria buscada.
        CategoriaSinData,
        /// Devuelto si el pago buscado nunca se registro.
        PagoNoRegistrado,
        /// Devuelto si el pago que se quiso pagar ya habia sido pagado con anterioridad.
        PagoYaPagado,
        /// Devuelto si el AccountId que llama no tiene permitido realizar la operacion, puesto que no posee los permisos necesarios.
        NoSePoseenLosPermisosSuficientes,
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
    /// 
    /// Las posibles categorias de los socios. 
    /// Para obtener el id de categoria se debe pasar el Enum a u32. 
    #[repr(u32)]
    #[derive(scale::Decode, scale::Encode,Debug, Clone, EnumCountMacro, EnumIter)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Categoria{ 
        A,
        B{deporte_seleccionado_por_el_usuario:Actividad},
        C,
    }
    impl Categoria {
        pub fn discriminant(&self) -> u32 {
            unsafe { *<*const _>::from(self).cast::<u32>() }
        }
    }
    
    /// Voy a llorar del asco que me da hacer "Cantidad". Pido disculpas por los malestares emocionales que esto pueda llegar a crear
    /// "Cantidad"  puede dar errores
    /// MODIFICAR. TERMINAR. PEDIR DISCULPAS 
    /// 
    /// Las posibles actividades del club. 
    /// Para obtener el id de categoria se debe pasar el Enum a u32. 
    #[repr(u32)]
    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq,EnumCountMacro,EnumIter,Default)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Actividad{
        #[default]
        Futbol,
        Gimnasio,
        Basquet,
        Rugby,
        Hockey,
        Natacion,
        Tenis,
        Paddle,
    }
    
    /// La informacion personal de un socio. 
    /// Guarda:
    /// 
    /// nombre del socio 
    /// 
    /// apellido del socio
    /// 
    /// dni del socio
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
        costo_mensual_en_tokens:u128,
        /// Las actividades accesibles base.
        actividades_accesibles_base: Vec<Actividad>,
    }
    impl DatosCategoria{
        /// Retorna el id de la categoria de los cuales estos datos son
        pub fn id (&self) -> u32{
            self.id
        }
        /// Retorna el costo mensual (en tokens) de la categoria de los cuales estos datos son
        pub fn costo_mensual_en_tokens (&self) -> u128{
            self.costo_mensual_en_tokens
        }
        /// Retorna las actividades base que la categoria, de los cuales estos datos son, tiene acceso
        pub fn actividades_accesibles_base (&self) ->  Vec<Actividad>{
            self.actividades_accesibles_base.clone()
        }
    }


    /// La informacion de un socio. 
    /// Guarda:
    /// 
    /// su id unica. Esta se calcula como el utimo id+1. 
    /// 
    /// la categoria del socio. Para obtener la informacion de la categoria se debera usar get_categoria_data. 
    /// 
    /// sus datos personales 
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    struct Socio{
        id:u32,
        categoria:Categoria,
        datos_personales:DatosPersonalesSocio
    }

    /// La informacion de un pago. 
    /// Guarda:
    /// 
    /// su id unica. Esta se calcula como el utimo id+1. 
    /// 
    /// el socio_id. Para obtener la informacion del socio se deberan usar los diferentes getters para cada uno de sus campos. CHEQUEAR. TERMINAR
    /// 
    /// la fecha_de_pago del socio al que le corresponde, si es que tiene. 
    /// 
    /// la fecha de vencimiento de este pago. 
    /// 
    /// el monto a pagar por el usuario, se calcula como el monto de la categoria del usuario, y se vera afectado si el pago tiene bonificacion
    /// 
    /// informacion sobre si este pago tiene o no bonificacion
    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq)]
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
            if porcentaje_de_descuento_por_bonificacion > 99 {porcentaje_de_descuento_por_bonificacion = 1} 
            
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
            csr.fecha_de_la_ultima_actualizacion = LocalDateTime::now().to_instant().seconds() as u64; 
            csr.pagos.set(&Vec::new());
            csr
        }

        // ----------- Metodos privados
        
        fn tiene_permiso(&self) -> bool{
            !self.politica_de_autorizacion_activada || self.es_duenio() || self.es_editor()
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
                TipoId::Pago => {return (self.pagos.get_or_default().len()) as u32},
                TipoId::Socio => {self.mapping_lens.socios +=1; return self.mapping_lens.socios},
            }
        }

        /// Analiza los pagos del socio y retorna true si cumple con las bonificaciones para que el proximo pago sea con bonificacion, false en caso contrario
        fn socio_cumple_las_condiciones_para_obtener_la_bonificacion(&self,socio_id:u32) -> bool{
            let pagos = self.pagos.get_or_default();

            let mut cant_pagos_del_usuario =0; 
            let mut cant_pagos_del_usuario_que_cumplen_la_condicion =0; 

            for pago in pagos.iter().rev() {
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
                return fecha_de_pago > pago.fecha_de_vencimiento
            }
            return LocalDateTime::now() > LocalDateTime::at(pago.fecha_de_vencimiento as i64);
        }

        /// Crea una cuota a vencer en fecha_de_vencimiento para el socio solicitado
        /// 
        /// Este metodo es privado del contrato
        /// 
        /// Posibles Error: SocioNoRegistrado, CategoriaSinData
        fn crear_cuota_para_socio(&mut self, socio_id:u32, fecha_de_vencimiento:LocalDateTime) -> Result<(),Error>{
            if !self.socios.contains(socio_id) {return Err(Error::SocioNoRegistrado)}

            let id_categoria_del_usuario = self.socios.get(socio_id).unwrap().categoria.discriminant();
            if !self.categorias_data.contains(id_categoria_del_usuario) {return Err(Error::CategoriaSinData);}

            let mut monto_cuota=self.categorias_data.get::<u32>(id_categoria_del_usuario).unwrap().costo_mensual_en_tokens;
            let cumple_las_condiciones_para_obtener_la_bonificacion = self.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id);
            if cumple_las_condiciones_para_obtener_la_bonificacion {monto_cuota -= monto_cuota*(self.porcentaje_de_descuento_por_bonificacion as u128)/100}

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

        /// Dado un dni retorna el socio, si es que existe
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
        fn listar_pagos_del_socio(&self, id_socio:u32) -> Vec<Pago>{
            let pagos = self.pagos.get_or_default();
            pagos.into_iter().filter(|p|p.socio_id == id_socio).collect()
        }

        fn SLICE_O_PAGINACION_QUE_ES_ESO(&self, vec:Vec<Pago>) -> Vec<Pago>{ // CONSULTAR
            vec
        }

        /// Retorna el primer pago sin pagar del usuario solicitado, si hay alguno
        fn get_primer_pago_sin_acreditar(&self, socio_id:u32) -> Option<Pago>{
            self.pagos.get_or_default().iter().find(|p|p.socio_id == socio_id && p.fecha_de_pago.is_none()).cloned()
        }


        fn marcar_pago_pagado(&mut self, pago_id:u32) -> Result<(),Error>{  // calculo que este tambien da para result
            let mut pagos = self.pagos.get_or_default();
            if pagos.len()<pago_id as usize {return Err(Error::PagoNoRegistrado);};

            if let Some(_) = pagos[pago_id as usize].fecha_de_pago.clone(){return Err(Error::PagoYaPagado);};

            pagos[pago_id as usize].fecha_de_pago = Some(LocalDateTime::now().to_instant().seconds() as u64);
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
        /// Posibles Error: NoSePoseenLosPermisosSuficientes
        #[ink(message)]
        pub fn cargar_data_categoria(&mut self, categoria:Categoria, costo_mensual_en_tokens:u128, actividades_accesibles_base: Vec<Actividad>) -> Result<(),Error>{ 
            self.cargar_data_categoria_priv(categoria,costo_mensual_en_tokens,actividades_accesibles_base)
        }
        fn cargar_data_categoria_priv(&mut self, categoria:Categoria, costo_mensual_en_tokens:u128, actividades_accesibles_base: Vec<Actividad>) -> Result<(),Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes);}
            self.categorias_data.insert(categoria.discriminant(),&DatosCategoria{id:categoria.discriminant(), costo_mensual_en_tokens,actividades_accesibles_base });
            Ok(())
        }

        /// Dado un AccountId, lo setea como nuevo duenio del Club
        /// 
        /// Solo puede ejecutarse si quien llama a este metodo es el duenio del Club
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes
        #[ink(message)]
        pub fn set_duenio (&mut self, nuevo_duenio_account_id:AccountId) -> Result<(),Error>{
            self.set_duenio_priv(nuevo_duenio_account_id)
        }
        fn set_duenio_priv(&mut self, nuevo_duenio_account_id:AccountId) -> Result<(),Error>{
            if !self.es_duenio(){ return Err(Error::NoSePoseenLosPermisosSuficientes)}
        
            self.duenio_account_id = nuevo_duenio_account_id; 
            return Ok(());
        }
        /// Dado un AccountId, lo agrega a la lista de editores autorizados 
        /// 
        /// Solo puede ejecutarse si quien llama a este metodo es el duenio del Club
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes
        #[ink(message)]
        pub fn autorizar_editor (&mut self, nuevo_editor:AccountId) -> Result<(),Error>{ 
            self.autorizar_editor_priv(nuevo_editor)
        }
        fn autorizar_editor_priv (&mut self, nuevo_editor:AccountId) -> Result<(),Error>{ 
            if !self.es_duenio(){ return Err(Error::NoSePoseenLosPermisosSuficientes)} 
            
            self.editores.insert(nuevo_editor.clone(),&nuevo_editor); 
            return Ok(());
        }

        /// Dado un AccountId, lo elimina de la lista de editores autorizados 
        /// 
        /// Solo puede ejecutarse si quien llama a este metodo es el duenio del Club
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes
        #[ink(message)]
        pub fn desautorizar_editor (&mut self, editor:AccountId) -> Result<(),Error>{ 
            self.desautorizar_editor_priv(editor)
        }
        fn desautorizar_editor_priv (&mut self, editor:AccountId) -> Result<(),Error>{
            if !self.es_duenio(){ return Err(Error::NoSePoseenLosPermisosSuficientes)}
            
            self.editores.remove(editor);
            return Ok(());
        }

        /// Dada una cantidad, la setea como nueva cantidad de pagos consecutivos sin atrasos necesarios para descuento
        /// 
        /// Se necesitan permisos para ejecutar
        /// 
        /// Posibles result: NoSePoseenLosPermisosSuficientes
        #[ink(message)]
        pub fn set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento(&mut self,cant:u32) -> Result<(),Error>{  
            self.set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento_priv(cant)
        }
        fn set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento_priv(&mut self,cant:u32) -> Result<(),Error>{ 
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes);}

            self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento=cant;

            return Ok(());
        }

        /// Dados un nombre, apellido, dni y una categoria, registra un nuevo socio y le crea una primer cuota a vencer dentro de 10 dias
        /// 
        /// Se necesitan permisos para ejecutar
        /// 
        /// Posibles result: NoSePoseenLosPermisosSuficientes, CategoriaSinData, SocioNoRegistrado. 
        /// SocioNoRegistrado aparece si falla la creacion de la primer cuota
        #[ink(message)]
        pub fn registrar_nuevo_socio(&mut self,nombre:String,apellido:String,dni:u32,categoria:Categoria) -> Result<(),Error>{
            self.registrar_nuevo_socio_priv(nombre, apellido, dni, categoria)
        }
        fn registrar_nuevo_socio_priv(&mut self,nombre:String,apellido:String,dni:u32,categoria:Categoria) -> Result<(),Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes);}
            if !self.categorias_data.contains(categoria.discriminant()) {return Err(Error::CategoriaSinData);}

            let info_personal_del_socio=DatosPersonalesSocio{nombre,apellido,dni:dni.clone()};
            let socio=Socio{id:self.nueva_id(TipoId::Socio),categoria,datos_personales:info_personal_del_socio};
            self.socios.insert(socio.id, &socio);

            return self.crear_cuota_para_socio(socio.id, LocalDateTime::now().add_seconds(604800)); 
                                        // reemplazar 604800 por -> 10*(constante cantidad_segundos_por_dia)
        }

        /// Es momento de otra actualizacion si en este mes no fue hecha otra actualizacion
        fn es_momento_de_otra_actualizacion(&self) -> bool{
            !(LocalDateTime::now().date().month() == LocalDateTime::at(self.fecha_de_la_ultima_actualizacion as i64).date().month())
        }

        /// Crea una cuota a vencer el dia 10 del mes actual para todos los usuarios. 
        /// 
        /// Este metodo se puede llamar manualmente, y ademas se llama cada vez que se quiere realizar un pago. 
        /// Este metodo se ejecutara como mucho una vez por mes. De ser llamado mas veces devolvera un Result. 
        /// 
        /// Se necesitan permisos para ejecutar
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, NoTranscurrioElTiempoNecesarioDesdeElUltimoLlamado, CategoriaSinData, 
        #[ink(message)]
        pub fn actualizacion_mensual(&mut self) ->Result<(),Error>{
            self.actualizacion_mensual_priv()
        }
        fn actualizacion_mensual_priv(&mut self) ->Result<(),Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes);}
            if !self.es_momento_de_otra_actualizacion() {return Err(Error::NoTranscurrioElTiempoNecesarioDesdeElUltimoLlamado);}

            let ahora = LocalDateTime::now();
            for i in 1..self.mapping_lens.socios+1 {
                match self.crear_cuota_para_socio(i, LocalDateTime::new(LocalDate::ymd(ahora.date().year(),ahora.date().month(),10).unwrap(),LocalTime::midnight())){
                    Ok(_)=>{},
                    Err(error)=>{return Err(error)}
                } 
            }
            self.fecha_de_la_ultima_actualizacion = ahora.to_instant().seconds() as u64;
            Ok(())
        }

        /// Dados un dni y un monto, marca como pagado el pago sin pagar mas viejo
        /// 
        /// Este metodo tambien ejecuta actualizacion mensual
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, SocioNoRegistrado, SocioNoPoseePagosSinAcreditar, MontoInvalido
        #[ink(message)]
        pub fn registrar_nuevo_pago(&mut self, dni_socio:u32, monto:u128 ) -> Result<(),Error>{
            self.registrar_nuevo_pago_priv(dni_socio,monto)
        }
        fn registrar_nuevo_pago_priv(&mut self, dni_socio:u32, monto:u128 ) -> Result<(),Error>{
            match self.actualizacion_mensual(){
                Ok(_)=>{},
                Err(_)=>{}
            }
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes);}
            let Some(socio) = self.get_socio_dni(dni_socio) else {return Err(Error::SocioNoRegistrado);};
            
            let Some(pago) = self.get_primer_pago_sin_acreditar(socio.id) else {return Err(Error::SocioNoPoseePagosSinAcreditar);};
            if !(pago.monto == monto) {return Err(Error::MontoInvalido);};
            self.marcar_pago_pagado(pago.id)
        }

        /// Activa la politica de autorizacion. 
        /// Solo puede ejecutarse si quien llama a este metodo es el duenio del Club
        #[ink(message)]
        pub fn activar_politica_de_autorizacion (&mut self) -> Result<(),Error>{ //TERMINAR. Devolver Result
            self.activar_politica_de_autorizacion_priv()
        }
        fn activar_politica_de_autorizacion_priv (&mut self) -> Result<(),Error>{ //TERMINAR. Devolver Result
            if !self.es_duenio(){return Err(Error::NoSePoseenLosPermisosSuficientes)}
            
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
            if !self.es_duenio(){return Err(Error::NoSePoseenLosPermisosSuficientes)}
            
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
            Categoria::COUNT as u32
        }

        /// CONSULTAR. ARREGLAR. TERMINAR. MODIFICAR. PEDIR DISCULPAS
        /// 
        /// Retorna la cantidad de actividades posibles del Club
        #[ink(message)]
        pub fn cant_actividades(&self) -> u32 {
            self.cant_actividades_priv()
        }
        fn cant_actividades_priv(&self) -> u32 {
            Actividad::COUNT as u32
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
        pub fn tengo_permisos_suficientes_para_realizar_operaciones(&self) -> bool{
            self.tengo_permisos_suficientes_para_realizar_operaciones_priv()
        }
        fn tengo_permisos_suficientes_para_realizar_operaciones_priv(&self) -> bool{
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

        /// Dado un dni se listan sus pagos. 
        /// 
        /// Si se ingresa None, se listaran los pagos de todos usuarios se han realizado al Club
        /// 
        /// Si se ingresa Some(dni), solo se listaran los pagos del usuario cuyo dni haya sido ingresado. 
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, SocioNoRegistrado
        #[ink(message)]
        pub fn get_pagos_de(&self, dni_ingresado:u32)-> Result<Vec<Pago>,Error>{ 
            self.get_pagos_de_priv(dni_ingresado)
        }
        fn get_pagos_de_priv(&self, dni_ingresado:u32)->Result<Vec<Pago>,Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes)}
            let Some(socio) = self.get_socio_dni(dni_ingresado) else {return Err(Error::SocioNoRegistrado)};
            let pagos_de_un_socio = self.listar_pagos_del_socio(socio.id);
            Ok(pagos_de_un_socio)
        }

        /// Retorna todos los pagos que se han realizado al Club
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes
        #[ink(message)]
        pub fn get_pagos(&self) -> Result<Vec<Pago>,Error>{
            self.get_pagos_priv()
        }
        fn get_pagos_priv(&self) -> Result<Vec<Pago>,Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes)}
            let todos_los_pagos = self.pagos.get_or_default(); //CONSULTA: tambien podria ser self.pagos.get();
            Ok(self.SLICE_O_PAGINACION_QUE_ES_ESO(todos_los_pagos))
        }

        /// Retorna una lista de los pagos del mes y anio indicados
        /// 
        /// Si en la fecha no se realizaron pagos, se retornara una lista vacia
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes
        #[ink(message)]
        pub fn get_pagos_del_mes_y_anio(&self,fecha:Timestamp) -> Result<Vec<Pago>,Error>{
            self.get_pagos_del_mes_y_anio_priv(fecha)
        }
        fn get_pagos_del_mes_y_anio_priv(&self,fecha:Timestamp) -> Result<Vec<Pago>,Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes)}
            let fecha = LocalDateTime::at(fecha as i64);
            let pagos_del_mes_y_anio = self.pagos.get_or_default().into_iter().filter(|p|
                LocalDateTime::at(p.fecha_de_vencimiento as i64).date().month() == fecha.date().month() &&
                LocalDateTime::at(p.fecha_de_vencimiento as i64).date().year() == fecha.date().year())      .collect();
            Ok(pagos_del_mes_y_anio)
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
        /// TERMINAR: HACER LO MISMO PARA LOS DEMAS CAMPOS DE SOCIO
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

        /// Devuelve el primer pago sin acreditar
        /// 
        /// Posibles Error: SocioNoRegistrado, SocioNoPoseePagosSinAcreditar
        #[ink(message)]
        pub fn get_primer_pago_sin_acreditar_del_socio_dni(&self, socio_dni:u32) -> Result<Pago,Error>{
            let Some(socio) = self.get_socio_dni(socio_dni) else {return Err(Error::SocioNoRegistrado)};
            let Some(pago) = self.get_primer_pago_sin_acreditar(socio.id) else {return Err(Error::SocioNoPoseePagosSinAcreditar)};
            Ok(pago)
        } 

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
        // 1- Como hacemos para saber la longitud de un enum?
    // TERMINAR:
        // 1- hacer los getters para todos los campos de un socio, que reciban un socio_id. Para obtener el socio_id_es con el dni
        // 2- Cuando se reciben enums chequear que no se haya elegido "Cantidad", ya que no es una opcion valida 
        // 3- Chequear que la confirmacion de los permisos esta en todos los metodos
        // 4- Fijarse si cuando se usa get_socio_dni, se usa solo para obtener el ID. CAMBIARLO A OBTENER EL ID Y HACERLO PUBLICO, PARA OBTENER EL SOCIO CON EL ID SE OBTIENE

/// Module ink_env::test -> https://paritytech.github.io/ink/ink_env/test/index.html
/// Examples -> https://github.com/paritytech/ink-examples/blob/main/erc20/lib.rs
    #[cfg(test)]
    mod tests {
        use datetime::Month;

        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        //-------------------------- modulos de inicializacion para evitar repeticion en los test ----------------------
        
        
        /// Crea el club con valores cualquiera
        /// 
        /// Tomamos a Alice como la duenia del Club
        fn crear_club_sem_rust()->ClubSemRust{
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

            // se crea y carga la categoria a
            let mut vec_de_actividades_a=Vec::new();
            vec_de_actividades_a.push(Actividad::Natacion);
            vec_de_actividades_a.push(Actividad::Futbol);
            vec_de_actividades_a.push(Actividad::Gimnasio);
            vec_de_actividades_a.push(Actividad::Basquet);
            vec_de_actividades_a.push(Actividad::Rugby);
            vec_de_actividades_a.push(Actividad::Hockey);
            vec_de_actividades_a.push(Actividad::Tenis);
            vec_de_actividades_a.push(Actividad::Paddle);
            
            assert!(club.cargar_data_categoria(Categoria::A, 5000, vec_de_actividades_a.clone()).is_ok());

            // chequeamos si los datos se guardaron correctamente en la categoria a
            let categoria_data_a = club.get_categoria_data(Categoria::A.discriminant());
            
            assert!(categoria_data_a.is_ok());
            let categoria_data_a = categoria_data_a.unwrap();
            assert_eq!((categoria_data_a.actividades_accesibles_base()),vec_de_actividades_a);
            assert_eq!(categoria_data_a.id(),Categoria::A.discriminant());
            assert_eq!(categoria_data_a.costo_mensual_en_tokens(),5000);


            // se crea y carga la categoria b
            let mut vec_de_actividades_b=Vec::new();
            vec_de_actividades_b.push(Actividad::Gimnasio);
            // la actividad puede ser cualquiera, no se va a guargar, es solo para obtener el indice
            assert!(club.cargar_data_categoria(Categoria::B { deporte_seleccionado_por_el_usuario:Actividad::default() }, 3000, vec_de_actividades_b.clone()).is_ok());

            // chequeamos si los datos se guardaron correctamente en la categoria b
            let categoria_data_b = club.get_categoria_data(Categoria::B { deporte_seleccionado_por_el_usuario:Actividad::default() }.discriminant());
                        
            assert!(categoria_data_b.is_ok());
            let categoria_data_b = categoria_data_b.unwrap();
            assert_eq!((categoria_data_b.actividades_accesibles_base()),vec_de_actividades_b);
            assert_eq!(categoria_data_b.id(),Categoria::B { deporte_seleccionado_por_el_usuario:Actividad::default()}.discriminant());
            assert_eq!(categoria_data_b.costo_mensual_en_tokens(),3000);


            // se crea y carga la categoria c
            let mut vec_de_actividades_c=Vec::new();
            vec_de_actividades_c.push(Actividad::Gimnasio);
            assert!(club.cargar_data_categoria(Categoria::C,2000,vec_de_actividades_c.clone()).is_ok());

            // chequeamos si los datos se guardaron correctamente en la categoria c
            let categoria_data_c = club.get_categoria_data(Categoria::C.discriminant());
            
            assert!(categoria_data_c.is_ok());
            let categoria_data_c = categoria_data_c.unwrap();
            assert_eq!((categoria_data_c.actividades_accesibles_base()),vec_de_actividades_c);
            assert_eq!(categoria_data_c.id(),Categoria::C.discriminant());
            assert_eq!(categoria_data_c.costo_mensual_en_tokens(),2000);

            assert!(club.todas_las_categorias_tienen_sus_datas_cargadas());
        }

        //-------------------------- tests ----------------------

        #[ink::test]
        fn test_get_ids_categorias() {
            let club = crear_club_sem_rust();
            
            let mut ids = Vec::new();
            for i in 0..Categoria::COUNT as u32{
                ids.push(i);
            }
            assert_eq!(ids,club.get_ids_categorias());
        }
        #[ink::test]
        fn test_get_ids_actividades() {
            let club = crear_club_sem_rust();

            let mut ids = Vec::new();
            for i in 0..Actividad::COUNT as u32{
                ids.push(i);
            }
            assert_eq!(ids,club.get_ids_actividades());
        }
        
        #[ink::test]
        fn test_existe_actividad_id () {
            let club = crear_club_sem_rust();
            assert!(club.existe_actividad_id(Actividad::Basquet as u32));
            assert!(club.existe_actividad_id(Actividad::Natacion as u32));
            assert!(club.existe_actividad_id(Actividad::Futbol as u32));
            assert!(!club.existe_actividad_id(800));
        }
        #[ink::test]
        fn test_existe_categoria_id (){
            let club = crear_club_sem_rust();
            assert!(club.existe_categoria_id(Categoria::A.discriminant()));
            assert!(club.existe_categoria_id(Categoria::B{deporte_seleccionado_por_el_usuario: Actividad::default()}.discriminant()));
            assert!(club.existe_categoria_id(Categoria::C.discriminant()));
            assert!(!club.existe_categoria_id(800));
        }
        
        #[ink::test]
        fn test_discriminant(){
            assert_eq!(Categoria::A.discriminant(),0);
            // no importa el deporte, solo es para obtener el enum
            assert_eq!( Categoria::B { deporte_seleccionado_por_el_usuario: (Actividad::default())}.discriminant(),1);
            assert_eq!(Categoria::C.discriminant(),2);
        }
        #[ink::test]
        fn test_creacion_club_sem_rust(){
            let club = crear_club_sem_rust();
            // Transfer event triggered during initial construction.
            //let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
            //assert_eq!(1, emitted_events.len()); // que seria esto?
        }
        #[ink::test]
        fn test_soy_duenio_siendo_duenio(){
            let club = crear_club_sem_rust();
            assert!(club.soy_duenio());
        }
        #[ink::test]
        fn test_autorizar_editor_y_soy_duenio_siendo_staff(){
            let mut club = crear_club_sem_rust();
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
            let club = crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let cliente=accounts.charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(cliente);
            assert!(!club.soy_duenio());
        }
        #[ink::test]
        fn tengo_permisos_suficientes_para_realizar_operaciones_siendo_duenio(){
            let club = crear_club_sem_rust();
            assert!(club.tengo_permisos_suficientes_para_realizar_operaciones());
        }
        #[ink::test]
        fn tengo_permisos_suficientes_para_realizar_operaciones_siendo_staff(){
            let mut club = crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            let staff=accounts.bob;
            assert!(club.autorizar_editor(staff).is_ok());
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);
            assert!(club.tengo_permisos_suficientes_para_realizar_operaciones());
        }
        #[ink::test]
        fn test_tengo_permisos_suficientes_para_realizar_operaciones_siendo_cliente_con_politica_de_autorizacion_activada_y_desactivada(){
            let mut club = crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let duenio=accounts.alice;
            let staff=accounts.bob;
            let cliente=accounts.charlie;
            assert!(club.autorizar_editor(staff).is_ok());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);
            assert!(club.tengo_permisos_suficientes_para_realizar_operaciones());
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(cliente);
            assert!(!club.tengo_permisos_suficientes_para_realizar_operaciones());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(duenio);
            assert!(club.desactivar_politica_de_autorizacion().is_ok());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);
            assert!(club.tengo_permisos_suficientes_para_realizar_operaciones());
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(cliente);
            assert!(club.tengo_permisos_suficientes_para_realizar_operaciones());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(duenio);
            assert!(club.activar_politica_de_autorizacion().is_ok());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);
            assert!(club.tengo_permisos_suficientes_para_realizar_operaciones());
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(cliente);
            assert!(!club.tengo_permisos_suficientes_para_realizar_operaciones());

        }
        #[ink::test]
        fn test_get_cantidad_de_pagos_necesarios_para_bonificacion(){
            let mut club = crear_club_sem_rust();
            assert_eq!(club.get_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento(),3);
        }
        #[ink::test]
        fn test_cargar_data_categoria_y_get_data_categoria(){
            let mut club = crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            crear_y_cargar_categorias(&mut club);
        }
        #[ink::test]
        fn test_existe_socio_dni_y_registrar_nuevo_socio(){
            let mut club = crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            crear_y_cargar_categorias(&mut club);
            let dni = 12;
            assert!(!club.existe_socio_dni(dni));
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni, Categoria::B { deporte_seleccionado_por_el_usuario: (Actividad::Futbol) }).is_ok());
            assert!(club.existe_socio_dni(dni));
        }
        #[ink::test]
        fn test_tengo_permisos_suficientes_para_realizar_operaciones_siendo_cliente(){
            let mut club = crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            
            let cliente=accounts.charlie;
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(cliente);
            assert!(!club.tengo_permisos_suficientes_para_realizar_operaciones())
        }
        #[ink::test]
        fn test_desautorizar_editor(){
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let duenio = accounts.alice;
            let mut club= ClubSemRust::new(duenio,3,10,true);
            let staff=accounts.bob;

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);
            assert!(!club.tengo_permisos_suficientes_para_realizar_operaciones());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(duenio);
            assert!(club.autorizar_editor(staff).is_ok());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);
            assert!(club.tengo_permisos_suficientes_para_realizar_operaciones());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(duenio);
            assert!(club.desautorizar_editor(staff).is_ok());

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(staff);
            assert!(!club.tengo_permisos_suficientes_para_realizar_operaciones());
        }
        #[ink::test]
        fn test_set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento(){
            let mut club = crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert!(club.set_cant_pagos_consecutivos_sin_atrasos_necesarios_paga_descuento(5).is_ok());
            assert_eq!(club.get_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento(),5);
        }
        #[ink::test]
        fn test_actualizacion_mensual(){
            let mut club = crear_club_sem_rust();
            crear_y_cargar_categorias(&mut club);

            assert_eq!(club.actualizacion_mensual().err().unwrap(),Error::NoTranscurrioElTiempoNecesarioDesdeElUltimoLlamado);
            // Forzamos a que ya sea momento de otra actualizacion, restando los segundos en un mes a la fecha de la ultima actualizacion
            club.fecha_de_la_ultima_actualizacion-= 2678400;
            assert!(club.actualizacion_mensual().is_ok());
        }
        #[ink::test]
        fn test_get_pagos(){
            let mut club=crear_club_sem_rust();
            crear_y_cargar_categorias(&mut club);
            let dni1 = 90;
            let dni2=30;
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni1, Categoria::B { deporte_seleccionado_por_el_usuario: (Actividad::Futbol) }).is_ok());
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Perri".to_string(),dni2, Categoria::C).is_ok());
            let pagos_totales=club.get_pagos();
            let mut pagos_totales_manual=Vec::new();
            pagos_totales_manual.push(club.get_pagos_de(dni1).unwrap().pop().unwrap());
            pagos_totales_manual.push(club.get_pagos_de(dni2).unwrap().pop().unwrap());
            assert_eq!(pagos_totales.unwrap(),pagos_totales_manual);
        }
        /*id:u32,
        socio_id: u32,
        fecha_de_pago:Option<Timestamp>,
        fecha_de_vencimiento:Timestamp,
        monto:u128,
        tiene_bonificacion:bool */
        #[ink::test]
        fn test_get_pagos_de(){
            let mut club=crear_club_sem_rust();
            crear_y_cargar_categorias(&mut club);
            let dni1 = 90;
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni1, Categoria::B { deporte_seleccionado_por_el_usuario: (Actividad::Futbol) }).is_ok());
            let pago_de_charlie=club.get_pagos_de(dni1).unwrap();
            let pago_de_charlie_manual=club.get_pagos().unwrap();
            assert_eq!(pago_de_charlie,pago_de_charlie_manual)
        }
        #[ink::test]
        fn test_set_duenio(){
            let mut club=crear_club_sem_rust();
            crear_y_cargar_categorias(&mut club);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let nuevo_duenio=accounts.bob;
            club.set_duenio(accounts.bob);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(nuevo_duenio);
            assert!(club.soy_duenio());
        }
        #[ink::test]
        fn test_get_pagos_del_mes_y_anio(){
            let mut club=crear_club_sem_rust();
            crear_y_cargar_categorias(&mut club);

            let mes_y_anio_de_venicimiento_buscados = LocalDateTime::new(
                LocalDate::ymd(2021,Month::October,10).unwrap(),  
                LocalTime::midnight() 
            );
            let fecha_de_vencimiento_no_buscada =  LocalDateTime::now();

            let dni1 = 90;
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni1, Categoria::A).is_ok());
            assert!((club.crear_cuota_para_socio(club.get_socio_dni(dni1).unwrap().id,mes_y_anio_de_venicimiento_buscados)).is_ok());
            assert!((club.crear_cuota_para_socio(club.get_socio_dni(dni1).unwrap().id,fecha_de_vencimiento_no_buscada)).is_ok());
            
            let dni2 = 57;
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni2, Categoria::B { deporte_seleccionado_por_el_usuario: (Actividad::Futbol) }).is_ok());
            assert!((club.crear_cuota_para_socio(club.get_socio_dni(dni2).unwrap().id,mes_y_anio_de_venicimiento_buscados)).is_ok());
            assert!((club.crear_cuota_para_socio(club.get_socio_dni(dni2).unwrap().id,fecha_de_vencimiento_no_buscada)).is_ok());

            let pagos_del_club=club.get_pagos().unwrap();
            let pagos_anio_y_mes = club.get_pagos_del_mes_y_anio(mes_y_anio_de_venicimiento_buscados.to_instant().seconds() as u64).unwrap();
            assert_eq!(pagos_anio_y_mes, 
                        pagos_del_club.into_iter().filter(|p|
                                            LocalDateTime::at(p.fecha_de_vencimiento as i64).date().month() == mes_y_anio_de_venicimiento_buscados.date().month() &&
                                            LocalDateTime::at(p.fecha_de_vencimiento as i64).date().year() == mes_y_anio_de_venicimiento_buscados.date().year()) .collect::<Vec<Pago>>()
                                        );
        }
        #[ink::test]
        fn test_get_primer_pago_sin_acreditar_y_registrar_nuevo_pago(){
            let mut club = crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            crear_y_cargar_categorias(&mut club);
            
            let dni = 75;
            // al registrar un nuevo socio se le crea una cuota sin pagar
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni, Categoria::B { deporte_seleccionado_por_el_usuario: (Actividad::Futbol) }).is_ok());
            
            // obtengo el primer pago sin pagar. Ya que no paso un mes desde la creacion del club, no se creara otra cuota
            let pago = club.get_primer_pago_sin_acreditar_del_socio_dni(dni);
            assert!(pago.is_ok());
            let pago = pago.unwrap();
            // pagos_del_usuario_originales contiene el primer pago sin pagar.
            let pagos_del_usuario_originales = vec![pago.clone()];
            assert_eq!(club.get_pagos_de(dni).unwrap(),pagos_del_usuario_originales);
            assert!(club.registrar_nuevo_pago(dni, pago.monto()).is_ok());
            // Si registrar_nuevo_pago() funciona correctamente, luego de ejecutarlo no deverian ser iguales
            assert_ne!(club.get_pagos_de(dni).unwrap(),pagos_del_usuario_originales);

            // no tendria que tener pagos sin pagar
            assert_eq!(club.get_primer_pago_sin_acreditar_del_socio_dni(dni).err().unwrap(),Error::SocioNoPoseePagosSinAcreditar);

            let pagos =club.get_pagos_de(dni);
            assert!(pagos.is_ok());
            let pagos = pagos.unwrap();
            // hay un solo pago de ese usuario
            assert_eq!(pagos.len(),1);
            let pago = &pagos[0];
            // el pago tendria que tener fecha de pago
            assert!(pago.fecha_de_pago().is_some());
        }

        #[ink::test]
        fn test_crear_cuota_para_socio(){
            let mut club = crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            crear_y_cargar_categorias(&mut club);
            let dni = 8;
            club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni, Categoria::C);

            let fecha_de_vencimiento =  LocalDateTime::now().to_instant().seconds() + 604800;
            assert!((club.crear_cuota_para_socio(club.get_socio_dni(dni).unwrap().id,LocalDateTime::at(fecha_de_vencimiento as i64))).is_ok());
            
            let pagos =club.get_pagos_de(dni);
            assert!(pagos.is_ok());
            let pagos = pagos.unwrap();
            // hay 2 pagos de ese usuario
            assert_eq!(pagos.len(),2);
            let pago0 = &pagos[0];
            let pago1 = &pagos[1];

            assert_eq!(pago1.id(),1);
            assert_eq!(pago1.socio_id(),1);
            assert!(pago1.fecha_de_pago().is_none());
            assert_eq!(pago1.fecha_de_vencimiento(),fecha_de_vencimiento as u64);
            assert!(!pago1.tiene_bonificacion());
            let monto = club.get_categoria_data(Categoria::C.discriminant()).unwrap().costo_mensual_en_tokens();
            assert_eq!(pago1.monto(),monto);
        }
        #[ink::test]
        fn test_socio_cumple_las_condiciones_para_obtener_la_bonificacion(){
            let mut club = crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            crear_y_cargar_categorias(&mut club);
            // se tienen q pagar 3 cuotas al dia para obtener bonificacion
            assert_eq!(club.get_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento(),3);
            assert_eq!(club.pagos.get_or_default().len(),0);
            
            let dni = 75;
            // al registrar un nuevo socio se le crea una cuota sin pagar
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni, Categoria::B { deporte_seleccionado_por_el_usuario: (Actividad::Futbol) }).is_ok());
            let socio_id = club.get_socio_dni(dni).unwrap().id;
            
            assert_eq!(club.pagos.get_or_default().len(),1);

            // no tiene bonificacion todavia
            assert!(!club.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id));

            // paga la primer cuota al dia
            let pago = club.get_primer_pago_sin_acreditar_del_socio_dni(dni).unwrap();
            assert!(!pago.tiene_bonificacion());
            assert!(!club.pago_esta_vencido(&pago));
            assert!(club.registrar_nuevo_pago(dni, pago.monto()).is_ok());
            assert!(!club.pago_esta_vencido(&pago));

            // no tiene bonificacion todavia
            assert!(!club.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id));

            // se le agregan 2 cuotas y las paga al dia
            let fecha_de_vencimiento =  LocalDateTime::now().to_instant().seconds() + 604800;
            assert!((club.crear_cuota_para_socio(club.get_socio_dni(dni).unwrap().id,LocalDateTime::at(fecha_de_vencimiento as i64))).is_ok());
            assert_eq!(club.pagos.get_or_default().len(),2);
            let pago = club.get_primer_pago_sin_acreditar_del_socio_dni(dni).unwrap();
            assert!(!pago.tiene_bonificacion());
            //assert!(!club.pago_esta_vencido(&pago));
            assert!(club.registrar_nuevo_pago(dni, pago.monto()).is_ok());
            assert!(!club.pago_esta_vencido(&pago));

            // no tiene bonificacion todavia
            assert!(!club.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id));

            let fecha_de_vencimiento =  LocalDateTime::now().to_instant().seconds() + 604800;
            assert!((club.crear_cuota_para_socio(club.get_socio_dni(dni).unwrap().id,LocalDateTime::at(fecha_de_vencimiento as i64))).is_ok());
            assert_eq!(club.pagos.get_or_default().len(),3);
            let pago = club.get_primer_pago_sin_acreditar_del_socio_dni(dni).unwrap();
            assert!(!pago.tiene_bonificacion());
            assert!(!club.pago_esta_vencido(&pago));
            assert!(club.registrar_nuevo_pago(dni, pago.monto()).is_ok());
            assert!(!club.pago_esta_vencido(&pago));

            // Ya que pago 3 cuotas sin que alguna se venciese, le corresponde bonificacion
            assert!(club.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id));
            
            // se le agrega otra cuota (esta tiene bonificacion) y la paga al dia
            let fecha_de_vencimiento =  LocalDateTime::now().to_instant().seconds() + 604800;
            assert!((club.crear_cuota_para_socio(club.get_socio_dni(dni).unwrap().id,LocalDateTime::at(fecha_de_vencimiento as i64))).is_ok());
            assert_eq!(club.pagos.get_or_default().len(),4);
            let pago = club.get_primer_pago_sin_acreditar_del_socio_dni(dni).unwrap();
            assert!(pago.tiene_bonificacion());
            assert!(!club.pago_esta_vencido(&pago));
            assert!(club.registrar_nuevo_pago(dni, pago.monto()).is_ok());
            assert!(!club.pago_esta_vencido(&pago));

            // aunque haya pagado 4 cuotas seguidas con al dia, ya que obtuvo bonificacion recientemente, no le corresponde bonificacion
            assert!(!club.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id));

        }
        #[ink::test]
        fn test_socio_tiene_permitida_la_asistencia_a_con_categoria_a(){
            let mut club=crear_club_sem_rust();
            crear_y_cargar_categorias(&mut club);
            let dni = 47;
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni, Categoria::A).is_ok());
            let socio_id = club.get_socio_dni(dni).unwrap().id;
            assert!(club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Gimnasio as u32).unwrap());
            assert!(club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Futbol as u32).unwrap());
            assert!(club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Basquet as u32).unwrap());
            assert!(club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Natacion as u32).unwrap());
            assert!(club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Hockey as u32).unwrap());
            assert!(club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Paddle as u32).unwrap());
            assert!(club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Rugby as u32).unwrap());
            assert!(club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Tenis as u32).unwrap());
        }
        #[ink::test]
        fn test_socio_tiene_permitida_la_asistencia_a_con_categoria_b(){
            let mut club=crear_club_sem_rust();
            crear_y_cargar_categorias(&mut club);
            let dni = 47;
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni, Categoria::B { deporte_seleccionado_por_el_usuario: (Actividad::Futbol) }).is_ok());
            let socio_id = club.get_socio_dni(dni).unwrap().id;
            assert!(club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Gimnasio as u32).unwrap());
            assert!(club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Futbol as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Basquet as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Natacion as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Hockey as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Paddle as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Rugby as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Tenis as u32).unwrap());
        }
        #[ink::test]
        fn test_socio_tiene_permitida_la_asistencia_a_con_categoria_c(){
            let mut club=crear_club_sem_rust();
            crear_y_cargar_categorias(&mut club);
            let dni = 47;
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni, Categoria::C).is_ok());
            let socio_id = club.get_socio_dni(dni).unwrap().id;
            assert!(club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Gimnasio as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Futbol as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Basquet as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Natacion as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Hockey as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Paddle as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Rugby as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Tenis as u32).unwrap());
        }
    }
}
