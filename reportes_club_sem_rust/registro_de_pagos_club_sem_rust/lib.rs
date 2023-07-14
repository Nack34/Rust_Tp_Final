#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::registro_de_pagos_club_sem_rust::ClubSemRustRef;
#[ink::contract]
mod registro_de_pagos_club_sem_rust {

    

    use ink::env::block_timestamp;
    use ink::storage::Mapping;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Lazy;



    // https://docs.rs/strum_macros/0.25.1/strum_macros/index.html
    //use strum::EnumCount;
    //use strum_macros::EnumCount as EnumCountMacro;
    //use strum_macros::EnumIter;
    // para remplazar Strum, como Strum simplemente era usado para saber la cantidad de variantes de los enum, lo que himos fue, en los metodos en los que se usaba COUNT, lo reemplazamos por en numero, siende ese nro, la cantidad de variantes del enum 

    // https://docs.rs/datetime/latest/datetime/index.html
    //use datetime::LocalDateTime;
    //use datetime::DatePiece;
    //use datetime::LocalDate;
    //use datetime::LocalTime;
    //use datetime::Month;
    // para reemplazar datetime, creamos los structs e implementamos su logica manualmente. No documentamos nada referente a datetime ya que es una copia del real crate datetime. Para leer sobre su documentacion dirigirse al link anterior

    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq,PartialOrd,Copy)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    struct LocalDateTime{
        local_date:LocalDate, local_time:LocalTime
    }
    impl LocalDateTime{
        pub fn new(local_date:LocalDate, local_time:LocalTime) -> LocalDateTime{
            LocalDateTime{local_date,local_time}
        }
        pub fn now(club:&ClubSemRust) -> LocalDateTime{
            LocalDateTime::at(club.ahora() as i64)
        }
        // to_instant no se usa para ninguna parte logica del programa, pero ya que se uso, para no modificar la logica es necesario que exista
        pub fn to_instant(&self) -> LocalDateTime{
            self.clone()
        }
        pub fn seconds(&self) -> Timestamp{
            self.local_date.cant_segundos() + self.local_time.segundos as u64
        }
        pub fn local_date_time_del_inicio_del_time_stamp()->LocalDateTime{
            LocalDateTime { local_date:LocalDate{year:1970,month:Month::January,day:1},local_time: LocalTime::midnight() }
        }
        pub fn at(seconds_since_1970:i64)-> LocalDateTime{
            let mut local_date_time_resultante=Self::local_date_time_del_inicio_del_time_stamp();
            let cant_dias: u32= (seconds_since_1970 / 86400) as u32; // 86400 cantidad de segundos en un dia
            let cant_segundos: u32= (seconds_since_1970 % 86400) as u32; // 86400 cantidad de segundos en un dia
            local_date_time_resultante.local_date.sumar_dias(cant_dias);
            local_date_time_resultante.local_time.segundos+=cant_segundos;
            local_date_time_resultante
        }
        pub fn date(&self)-> LocalDate{
            self.local_date.clone()
        }
        pub fn add_seconds(&mut self,seconds:u128) -> Self{
            let cantidad_de_dias=(seconds/86400) as u32;
            self.local_date.sumar_dias(cantidad_de_dias);
            self.clone()
        }
    }
    
    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq,PartialOrd,Copy)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    enum Month{
        January,
        February,
        March,
        April,
        May,
        June,
        July,
        August,
        September,
        October,
        November,
        December,
    }
    impl Month{
        pub fn from_one(mes:i8) ->Result<Self,Error>{
            match mes {
                1 => {return Ok(Month::January);}
                2 => {return Ok(Month::February);}
                3 => {return Ok(Month::March);}
                4 => {return Ok(Month::April);}
                5 => {return Ok(Month::May);}
                6 => {return Ok(Month::June);}
                7 => {return Ok(Month::July);}
                8 => {return Ok(Month::August);}
                9 => {return Ok(Month::September);}
                10 => {return Ok(Month::October);}
                11 => {return Ok(Month::November);}
                12 => {return Ok(Month::December);}
                _=> {return Err(Error::FechaInvalida);}
            }
        }
    }

    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq,PartialOrd,Copy)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    struct LocalDate{
        year:    i64,
        month:   Month,
        day:     i8,
    }
    impl LocalDate{
        pub fn ymd(year:i64, month:Month,day:i8) -> Result<LocalDate,Error>{
            if !Self::es_fecha_valida(year,month.clone(),day) { return Err(Error::FechaInvalida) }
            Ok(LocalDate{year,month,day})
        }
        
        pub fn year(&self) -> i64{
            self.year
        }
        pub fn month(&self)-> Month{
            self.month.clone()
        }

        fn es_bisiesto(anio:i64) -> bool{
            anio % 4 == 0 || (anio % 4 == 0 && anio % 100 == 0 && anio % 400 == 0) 
        }
        fn es_dia_valido(dia:i8, dia_max:i8) -> bool{
            dia>0 && dia<=dia_max
        }
        fn es_fecha_valida(anio:i64, mes:Month, dia:i8) -> bool{
            Self::es_dia_valido(dia,Self::max_dia(anio,mes.clone()))
        }
        fn sumar_dias(&mut self,dias:u32){
            for _i in 0..dias {
                self.day+=1;
                self.chequear_carry();
            }
        }
        fn chequear_carry(&mut self){
            if !Self::es_fecha_valida(self.year,self.month.clone(),self.day) {
                if !self.inc_mes().is_ok(){
                    self.month=Month::January;
                    self.year+=1;
                }
                self.day = 1;
            }
        }
        fn max_dia(anio:i64,mes:Month) -> i8{ 
            match mes{
                Month::February=> {if Self::es_bisiesto(anio) {29} else {28}},
                Month::January|Month::March|Month::May|Month::July|Month::August|Month::October|Month::December=>31,
                Month::April|Month::June|Month::September|Month::November=>30,
            }   
        }
        fn inc_mes(&mut self) -> Result<(),()>{
            match self.month{
                Month::January => {self.month =Month::February} 
                Month::February => {self.month =Month::March} 
                Month::March => {self.month =Month::April} 
                Month::April => {self.month =Month::May} 
                Month::May => {self.month =Month::June} 
                Month::June => {self.month =Month::July} 
                Month::July => {self.month =Month::August} 
                Month::August => {self.month =Month::September} 
                Month::September => {self.month =Month::October} 
                Month::October => {self.month =Month::November} 
                Month::November => {self.month =Month::December} 
                Month::December => {return Err(())} 
            }
            Ok(())
        }
        fn mes_ant(month:Month) -> Result<Month,()>{
            match month{
                Month::January => {Err(())} 
                Month::February => {Ok(Month::January)} 
                Month::March => {Ok(Month::February)} 
                Month::April => {Ok(Month::March)} 
                Month::May => {Ok(Month::April)} 
                Month::June => {Ok(Month::May)} 
                Month::July => {Ok(Month::June)} 
                Month::August => {Ok(Month::July)} 
                Month::September => {Ok(Month::August)} 
                Month::October => {Ok(Month::September)} 
                Month::November => {Ok(Month::October)} 
                Month::December => {Ok(Month::November)} 
            }
        }
        pub fn cant_segundos(&self) ->Timestamp{
            let segundos_en_un_dia = 86400; 
            self.cant_dias() * segundos_en_un_dia
        }
        fn cant_dias(&self) -> u64{
            self.cant_dias_rec(self.cantidad_de_meses(),self.year,self.month.clone(),self.day as u64) -1
        }
        fn cantidad_de_meses(&self) -> u64{
            (self.year -1970) as u64 * 12 + (self.month.clone() as u64 )
        }
        fn cant_dias_rec(&self,cant_meses_que_faltan_contar:u64,mut anio_a_contar:i64,mes_a_contar:Month,dias_sumados:u64) ->u64{
            if cant_meses_que_faltan_contar== 0{
                return dias_sumados
            }

            let binding: Result<Month, ()> = Self::mes_ant(mes_a_contar);
            let mes_anterior;
            if binding.is_err() {anio_a_contar-=1; mes_anterior=Month::December;}
            else {mes_anterior = binding.unwrap()}
            self.cant_dias_rec(cant_meses_que_faltan_contar-1,anio_a_contar,mes_anterior.clone(),dias_sumados+(Self::max_dia(anio_a_contar as i64,mes_anterior) as u64))
        }
    }
    
    // LocalTime no se usa para ninguna parte logica del programa, pero ya que se uso, para no modificar la logica es necesario que exista
    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq,PartialOrd,Copy)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    struct LocalTime{
        segundos:u32,
    }
    impl LocalTime{
        pub fn midnight() -> LocalTime{
            LocalTime{segundos:0}
        }
    }
























    /// Los posibles tipos de errores all llamar a los metodos del contrato
    /// 
    /// SocioNoRegistrado es devuelto si el socio buscado no se encuentra registrado en el club. 
    /// 
    /// ActividadInvalida es devuelto si el Id de actividad ingresado no representa una actividad del Club.
    /// 
    /// CategoriaInvalida es devuelto si el Id de categoria ingresado no representa una categoria del Club.
    /// 
    /// CategoriaSinData es devuelto si no se cargo la data de la categoria buscada.
    /// 
    /// PagoNoRegistrado es devuelto si el pago buscado nunca se registro.
    /// 
    /// PagoYaPagado es devuelto si el pago que se quiso pagar ya habia sido pagado con anterioridad.
    /// 
    /// NoSePoseenLosPermisosSuficienteses es devuelto si el AccountId que llama no tiene permitido realizar la operacion, puesto que no posee los permisos necesarios.
    /// 
    ///  NoTranscurrioElTiempoNecesarioDesdeElUltimoLlamadoes devuelto si no transcurrio el tiempo necesario para que este metodo vuelva a ejecutarse
    /// 
    /// SocioNoPoseePagosSinAcreditar es devuelto si se quiere pagar un pago de un socio que no posee pagos sin acreditar
    /// 
    /// MontoInvalido es devuelto si el monto ingresado no corresponde con el del pago a acreditar
    /// 
    /// FechaInvalida es devuelto si la fecha ingresada no corresponde a una fecha real
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
        /// Devuelto si la fecha ingresada no corresponde a una fecha real
        FechaInvalida
    }

    /// Las posibles categorias de los socios. 
    /// Para obtener el id de categoria se debe pasar el Enum a integer. 
    #[repr(u32)]
    #[derive(scale::Decode, scale::Encode,Debug, Clone, /*EnumCountMacro, EnumIter,*/PartialEq)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Categoria{ 
        A,
        B{deporte_seleccionado_por_el_usuario:Actividad},
        C,
    }
    impl Categoria {
        /// Retorna el enum convertido a u32
        /// 
        /// Ver mas en https://doc.rust-lang.org/std/mem/fn.discriminant.html
        fn discriminant(&self) -> u32 {
            unsafe { *<*const _>::from(self).cast::<u32>() }
        }
    }
    
    /// Las posibles actividades del club. 
    /// Para obtener el id de categoria se debe pasar el Enum a integer. 
    #[repr(u32)]
    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq,/*EnumCountMacro,EnumIter,*/Default)]
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
    /// nombre del socio 
        nombre:String,
    /// apellido del socio
        apellido:String,
    /// dni del socio
        dni:u32
    }

    /// La data de una categoria.
    /// 
    /// Es necesario cargar las datas de las categorias para que el club funcione correctamente.
    /// 
    /// Guarda:
    /// 
    /// su id unica. Esta se calcula como la posicion relativa de la categoria en el enum. 
    /// 
    /// el costo mensual en tokens de la catagoria.
    /// 
    /// Las actividades accesibles base. 
    /// Dependiendo de la categoria, podrian haber mas actividades, que dependan de la seleccion del socio. 
    /// Estas estaran guardadas en la informacion del socio
    #[derive(scale::Decode, scale::Encode,Debug,Clone)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    struct DatosCategoria{
        /// El id de la categoria.
        /// Se obtiene al cnvertir el enum en u32
        id:u32, 
        /// El costo mensual en tokens de la catagoria.
        costo_mensual_en_tokens:u128,
        /// Las actividades accesibles base.
        actividades_accesibles_base: Vec<Actividad>,
    }

    /// La informacion de un socio. 
    /// Guarda:
    /// 
    /// su id unica. Esta se calcula como el utimo id+1. 
    /// 
    /// la categoria del socio. 
    /// 
    /// sus datos personales. 
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    struct Socio{
        id:u128,
        categoria:Categoria,
        datos_personales:DatosPersonalesSocio
    }

    /// La informacion de un pago. 
    /// Guarda:
    /// 
    /// su id unica. Esta se calcula como el utimo id+1. 
    /// 
    /// el id del socio a quien le pertenece este pago.
    /// 
    /// la fecha de pago del socio al que le corresponde, si es que tiene. 
    /// 
    /// la fecha de vencimiento de este pago. 
    /// 
    /// el monto a pagar por el usuario, se calcula como el monto de la categoria del usuario, y se vera afectado si el pago tiene bonificacion
    /// 
    /// informacion sobre si este pago tiene o no bonificacion
    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    struct Pago{
        /// El id unico del pago
        id:u128,
        /// El id del socio que realizo o que debe realizar el pago
        socio_id: u128,
        /// La fecha en la que se realizo el pago del pago, si se realizo
        fecha_de_pago:Option<Timestamp>,
        /// La fecha de vencimiento del pago
        fecha_de_vencimiento:Timestamp,
        /// El monto del pago
        monto:u128,
        /// Retorna true si el pago fue con bonificacion, false en caso contrario
        tiene_bonificacion:bool
    }
    /// TipoId es usado para obtener las IDs
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    enum TipoId{
        Socio, Pago,
    }
    /// MappingLens es usado para obtener las IDs
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    struct MappingLens{
        socios:u128,
    }

    /// El Club 
    /// 
    /// Tiene informacion sobre:
    /// 
    /// la politica de autorizaciones. Se tendra en cuenta para saber si se tiene o no permiso para operar
    /// 
    /// el duenio. Es quien tiene el poder de realizar todas las operaciones.
    /// 
    /// los editores del contrato. Son los que tendran el permiso de realizar operaciones de ser necesaria una autorizacion
    /// 
    /// los socios del Club.
    /// 
    /// las datas de las categorias de los Socios
    /// 
    /// los pagos realizados por los socios al club
    /// 
    /// la cantidad de pagos consecutivos sin atrasos necesarios para descuento en las cuotas de los usuarios y el porcentaje de descuento que se les hara a las cuotas de los usuarios si tienen bonificacion 
    #[ink(storage)]
    pub struct ClubSemRust {
        /// Se podrá activar o desactivar esta política de autorización por parte del dueño del contrato. 
        /// Si está desactivada, cualquiera podrá realizar operaciones; si está activada, sólo las direcciones autorizadas podrán hacerlo
        politica_de_autorizacion_activada:bool,

        /// El duenio del contrato.
        /// 
        /// Tendrá el poder de siempre tener permiso para cualquier operacion. 
        /// Podra autorizar o desautorizar a los editores. 
        /// Podra activar o desactivar la politica de actorizacion. 
        /// El cargo puede cederse. 
        duenio_account_id:AccountId,

        /// Los editores del contrato.
        /// 
        /// Son los que tendran el permiso de realizar operaciones de ser necesaria una autorizacion.
        /// El duenio puede autorizarlos y desautorizarlos
        editores:Mapping<AccountId,AccountId>,

        /// Los socios del Club.
        socios:Mapping<u128,Socio>,

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

        /// ///////////////////////////////////////////         Constructor         //////////////////////////////////////////////
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
            csr.fecha_de_la_ultima_actualizacion = LocalDateTime::now(&csr).to_instant().seconds() as u64; 
            csr.pagos.set(&Vec::new());
            csr
        }






        //////////////////////////////////////////////         Metodos Auxiliares         //////////////////////////////////////////////

        #[cfg(not(test))] 
        pub fn ahora(&self) ->Timestamp{
            self.env().block_timestamp()
        }
        
        #[cfg(test)] 
        pub fn ahora(&self) ->Timestamp{
            45184555
        }
        /// Retorna true si el caller tiene permisos suficientes para la mayoria de operaciones
        fn tiene_permiso(&self) -> bool{
            !self.politica_de_autorizacion_activada || self.es_duenio() || self.es_editor()
        }
        /// Retorna true si el caller es el duenio
        fn es_duenio (&self) -> bool{
            self.duenio_account_id == self.env().caller()
        }
        /// Retorna true si el caller es editor
        fn es_editor (&self) -> bool{
            self.editores.contains(self.env().caller())
        }

        /// Retorna una nueva Id para el TipoId seleccionado. 
        fn nueva_id (&mut self,tipo_id:TipoId) -> u128{
            match tipo_id{
                TipoId::Pago => {return (self.pagos.get_or_default().len()) as u128},
                TipoId::Socio => {self.mapping_lens.socios +=1; return self.mapping_lens.socios},
            }
        }

        /// Analiza los pagos del socio y retorna true si cumple con las bonificaciones para que el proximo pago sea con bonificacion, false en caso contrario
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, SocioNoRegistrado
        fn socio_cumple_las_condiciones_para_obtener_la_bonificacion(&self,socio_id:u128) -> Result<bool,Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes);}
            if !self.existe_socio_con_id(socio_id) {return Err(Error::SocioNoRegistrado)}
            let pagos = self.get_pagos_del_socio_con_id(socio_id).unwrap();

            let mut cant_pagos_del_usuario =0; 
            let mut cant_pagos_del_usuario_que_cumplen_la_condicion =0; 

            for pago in pagos.iter().rev() {
                    cant_pagos_del_usuario+=1;                    
                    if !pago.tiene_bonificacion && !self.pago_esta_vencido(pago.id).unwrap(){
                        cant_pagos_del_usuario_que_cumplen_la_condicion+=1;
                    } 
                
                if cant_pagos_del_usuario==self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento ||
                    cant_pagos_del_usuario>cant_pagos_del_usuario_que_cumplen_la_condicion {break;}
            }
            
            Ok(cant_pagos_del_usuario_que_cumplen_la_condicion==self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento)
        }


        /// Crea una cuota a vencer en fecha_de_vencimiento para el socio solicitado
        /// 
        /// Este metodo es privado del contrato
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, SocioNoRegistrado, CategoriaSinData
        fn crear_cuota_para_socio(&mut self, socio_id:u128, fecha_de_vencimiento:LocalDateTime) -> Result<(),Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes);}
            if !self.existe_socio_con_id(socio_id) {return Err(Error::SocioNoRegistrado)}

            let id_categoria_del_usuario = self.get_socio_con_id(socio_id).unwrap().categoria.discriminant();
            if !self.categoria_tiene_sus_datos_cargados(id_categoria_del_usuario) {return Err(Error::CategoriaSinData);}

            let mut monto_cuota=self.get_categoria_datos(id_categoria_del_usuario).unwrap().costo_mensual_en_tokens;
            let cumple_las_condiciones_para_obtener_la_bonificacion = self.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id).unwrap();
            if cumple_las_condiciones_para_obtener_la_bonificacion {monto_cuota -= monto_cuota*(self.porcentaje_de_descuento_por_bonificacion as u128)/100}

            let cuota=Pago{id:self.nueva_id(TipoId::Pago),socio_id,fecha_de_pago:None,
                fecha_de_vencimiento:fecha_de_vencimiento.to_instant().seconds() as u64,
                monto:(monto_cuota as u128), 
                tiene_bonificacion:cumple_las_condiciones_para_obtener_la_bonificacion
            }; 
            
            let mut pagos = self.get_pagos().unwrap();
            pagos.push(cuota);
            self.pagos.set(&pagos);
            Ok(())
        }


        /// Dado el id de un socio, retorna el primer pago sin pagar del usuario solicitado, si hay alguno
        fn get_primer_pago_sin_acreditar(&self, socio_id:u128) -> Option<Pago>{
            self.pagos.get_or_default().iter().find(|p|p.socio_id == socio_id && p.fecha_de_pago.is_none()).cloned()
        }

        /// Dado el id de un pago, lo marca como pagado steando como "ahora" la fecha de pago
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, PagoNoRegistrado, PagoYaPagado
        fn marcar_pago_pagado(&mut self, pago_id:u128) -> Result<(),Error>{ 
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes);}
            let pago_id = pago_id as usize;
            let mut pagos = self.get_pagos().unwrap();
            if pagos.len()<=pago_id {return Err(Error::PagoNoRegistrado);};

            if pagos[pago_id].fecha_de_pago.is_some(){return Err(Error::PagoYaPagado);};

            pagos[pago_id].fecha_de_pago = Some(LocalDateTime::now(self).to_instant().seconds() as u64);
            self.pagos.set(&pagos);
            
            Ok(())
        }



        /// Dados un anio, mes y dia, se construlle un LocalDateTime con LocalTime en midnight
        ///  LocalDate::ymd(anio:i64, mes:i8,dia:i8)
        /// Posibles Error: FechaInvalida
        fn construir_fecha_midnight(anio:i64, mes:i8,dia:i8) -> Result<LocalDateTime,Error>{
            let binding = Month::from_one(mes);
            let Ok(mes) = binding else {return Err(Error::FechaInvalida)};

            let binding = LocalDate::ymd(anio,mes,dia);
            let Ok(local_date) = binding else {return Err(Error::FechaInvalida)};

            Ok(LocalDateTime::new(
                local_date,  
                LocalTime::midnight() 
            ))
        }

        /// Es momento de otra actualizacion si en este mes no fue hecha otra actualizacion
        fn es_momento_de_otra_actualizacion(&self) -> bool{
            !(LocalDateTime::now(self).date().month() == LocalDateTime::at(self.fecha_de_la_ultima_actualizacion as i64).date().month())
        }








        //////////////////////////////////////////////         Setters         //////////////////////////////////////////////



        
        
        // ---------------------- SOBRE EL QUIEN LLAMA A LOS METODOS (EL USUARIO)  ---------------------------



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


        // ---------------------- SOBRE EL CLUB ---------------------------

        /// Activa la politica de autorizacion. 
        /// 
        /// Solo puede ejecutarse si quien llama a este metodo es el duenio del Club
        /// 
        /// Posibles result: NoSePoseenLosPermisosSuficientes
        #[ink(message)]
        pub fn activar_politica_de_autorizacion (&mut self) -> Result<(),Error>{
            self.activar_politica_de_autorizacion_priv()
        }
        fn activar_politica_de_autorizacion_priv (&mut self) -> Result<(),Error>{
            if !self.es_duenio(){return Err(Error::NoSePoseenLosPermisosSuficientes)}
            
            self.politica_de_autorizacion_activada=true;
            Ok(())
        }

        /// Desactiva la politica de autorizacion. 
        /// 
        /// Solo puede ejecutarse si quien llama a este metodo es el duenio del Club
        /// 
        /// Posibles result: NoSePoseenLosPermisosSuficientes
        #[ink(message)]
        pub fn desactivar_politica_de_autorizacion (&mut self) -> Result<(),Error>{
            self.desactivar_politica_de_autorizacion_priv()
        }
        fn desactivar_politica_de_autorizacion_priv (&mut self) -> Result<(),Error>{
            if !self.es_duenio(){return Err(Error::NoSePoseenLosPermisosSuficientes)}
            
            self.politica_de_autorizacion_activada=false;
            Ok(())
        }

        
        /// Dada una cantidad, la setea como nueva cantidad de pagos consecutivos sin atrasos necesarios para descuento
        /// 
        /// Solo puede ejecutarse si quien llama a este metodo es el duenio del Club
        /// 
        /// Posibles result: NoSePoseenLosPermisosSuficientes
        #[ink(message)]
        pub fn set_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento(&mut self,cant:u32) -> Result<(),Error>{  
            self.set_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento_priv(cant)
        }
        fn set_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento_priv(&mut self,cant:u32) -> Result<(),Error>{ 
            if !self.es_duenio(){ return Err(Error::NoSePoseenLosPermisosSuficientes)} 

            self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento=cant;

            Ok(())
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


        // ---------------------- SOBRE LAS ACTIVIDADES  ---------------------------





        // ---------------------- SOBRE LAS CATEGORIAS  ---------------------------




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







        // ---------------------- SOBRE LOS PAGOS ---------------------------







        /// Dados el id de un socio y un monto, marca como pagado el pago sin pagar mas viejo
        /// 
        /// Se necesitan permisos
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, SocioNoRegistrado, SocioNoPoseePagosSinAcreditar, MontoInvalido
        /// 
        /// Este metodo tambien ejecuta actualizacion mensual
        #[ink(message)]
        pub fn registrar_nuevo_pago(&mut self, socio_id:u128, monto:u128 ) -> Result<(),Error>{
            self.registrar_nuevo_pago_priv(socio_id, monto)
        }
        fn registrar_nuevo_pago_priv(&mut self, socio_id:u128, monto:u128 ) -> Result<(),Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes);}
            if !self.existe_socio_con_id(socio_id) {return Err(Error::SocioNoRegistrado)}

            match self.actualizacion_mensual(){
                Ok(_)=>{},
                Err(_)=>{}
            }

            let Some(pago) = self.get_primer_pago_sin_acreditar(socio_id) else {return Err(Error::SocioNoPoseePagosSinAcreditar);};
            if !(pago.monto == monto) {return Err(Error::MontoInvalido);};
            self.marcar_pago_pagado(pago.id)

        }







        /// Crea una cuota a vencer el dia 10 del mes actual para todos los usuarios. 
        /// 
        /// Se necesitan permisos para ejecutar
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, NoTranscurrioElTiempoNecesarioDesdeElUltimoLlamado, CategoriaSinData, 
        /// 
        /// Este metodo se puede llamar manualmente, y ademas se llama cada vez que se quiere realizar un pago. 
        /// Este metodo se ejecutara como mucho una vez por mes, de ser llamado mas veces devolvera un Error. 
        #[ink(message)]
        pub fn actualizacion_mensual(&mut self) ->Result<(),Error>{
            self.actualizacion_mensual_priv()
        }
        fn actualizacion_mensual_priv(&mut self) ->Result<(),Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes);}
            if !self.es_momento_de_otra_actualizacion() {return Err(Error::NoTranscurrioElTiempoNecesarioDesdeElUltimoLlamado);}

            let ahora = LocalDateTime::now(self);
            for i in 1..self.mapping_lens.socios+1 {
                match self.crear_cuota_para_socio(i, LocalDateTime::new(LocalDate::ymd(ahora.date().year(),ahora.date().month(),10).unwrap(),LocalTime::midnight())){
                    Ok(_)=>{},
                    Err(error)=>{return Err(error)}
                } 
            }
            self.fecha_de_la_ultima_actualizacion = ahora.to_instant().seconds() as u64;
            Ok(())
        }














        // ---------------------- SOBRE EL SOCIO ---------------------------







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
            if !self.categoria_tiene_sus_datos_cargados(categoria.discriminant()) {return Err(Error::CategoriaSinData);}

            let info_personal_del_socio=DatosPersonalesSocio{nombre,apellido,dni:dni.clone()};
            let socio=Socio{id:self.nueva_id(TipoId::Socio),categoria,datos_personales:info_personal_del_socio};
            self.socios.insert(socio.id, &socio);

            return self.crear_cuota_para_socio(socio.id, LocalDateTime::now(self).add_seconds(604800)); 
        }











        //////////////////////////////////////////////         Getters         //////////////////////////////////////////////

        
        




        // ---------------------- SOBRE EL QUIEN LLAMA A LOS METODOS (EL USUARIO)  ---------------------------



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








        // ---------------------- SOBRE EL CLUB ---------------------------


        /// Devuelve la cantidad de pagos consecutivos sin atrasos necesarios para descuento
        #[ink(message)]
        pub fn get_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento(&mut self)->u32{
            self.get_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento_priv()
        }
        fn get_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento_priv(&mut self)->u32{
            self.cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento
        }







        // ---------------------- SOBRE LAS ACTIVIDADES  ---------------------------





        /// Dada una actividad, retorna su id
        #[ink(message)]
        pub fn get_actividad_id (&self, actividad:Actividad) -> u32{
            self.get_actividad_id_priv(actividad)
        }
        fn get_actividad_id_priv (&self, actividad:Actividad) -> u32{
            actividad as u32
        }

        /// Retorna todos los ids de categorias posibles del Club
        #[ink(message)]
        pub fn get_ids_actividades(&self) -> Vec<u32>{
            self.get_ids_actividades_priv()
        }
        fn get_ids_actividades_priv(&self) -> Vec<u32>{
            (0..self.cant_actividades()).collect()
        }
        /// Dado un id de actividad, retorna true si es una actividad valida, false en caso contrario
        #[ink(message)]
        pub fn existe_actividad_con_id (&self, id_actividad:u32) -> bool{
            self.existe_actividad_con_id_priv(id_actividad)
        }
        fn existe_actividad_con_id_priv (&self, id_actividad:u32) -> bool{
            self.get_ids_actividades().contains(&id_actividad)
        }

        /// Retorna la cantidad de actividades posibles del Club
        #[ink(message)]
        pub fn cant_actividades(&self) -> u32 {
            self.cant_actividades_priv()
        }
        fn cant_actividades_priv(&self) -> u32 {
            //Actividad::COUNT as u32
            8
        }






        // ---------------------- SOBRE LAS CATEGORIAS  ---------------------------



        /// Dada una categoria, retorna su id
        #[ink(message)]
        pub fn get_categoria_id (&self, categoria:Categoria) -> u32{
            self.get_categoria_id_priv(categoria)
        }
        fn get_categoria_id_priv (&self, categoria:Categoria) -> u32{
            categoria.discriminant()
        }
        /// Retorna todos los ids de actividades posibles del Club
        #[ink(message)]
        pub fn get_ids_categorias(&self) -> Vec<u32>{
            self.get_ids_categorias_priv()
        }
        fn get_ids_categorias_priv(&self) -> Vec<u32>{
            (0..self.cant_categorias()).collect()
        }
        /// Dado un id de categoria, retorna true si es una categoria valida, false en caso contrario
        #[ink(message)]
        pub fn existe_categoria_con_id (&self, id_categoria:u32) -> bool{
            self.existe_categoria_con_id_priv(id_categoria)
        }
        fn existe_categoria_con_id_priv (&self, id_categoria:u32) -> bool{
            self.get_ids_categorias().contains(&id_categoria)
        }
        
        /// Retorna la cantidad de categorias posibles de los socios del Club
        #[ink(message)]
        pub fn cant_categorias(&self) -> u32 {
            self.cant_categorias_priv()
        }
        fn cant_categorias_priv(&self) -> u32 {
            //Categoria::COUNT as u32
            3
        }


        /// Dado el id de una categoria, retorna true si esta tiene su data cargada, false en caso contrario
        #[ink(message)]
        pub fn categoria_tiene_sus_datos_cargados(&self,id_categoria: u32) ->bool{
            self.categoria_tiene_sus_datos_cargados_priv(id_categoria)
        }
        fn categoria_tiene_sus_datos_cargados_priv(&self,id_categoria: u32) ->bool{
            self.categorias_data.contains(id_categoria)
        }
        /// Retorna true si todas las categorias tienen sus datas cargadas, false en caso contrario
        #[ink(message)]
        pub fn todas_las_categorias_tienen_sus_datas_cargadas(&self) ->bool{
            self.todas_las_categorias_tienen_sus_datas_cargadas_priv()
        }
        fn todas_las_categorias_tienen_sus_datas_cargadas_priv(&self) ->bool{
            let mut res = true;
            self.get_ids_categorias().into_iter().for_each(|c| res = res && self.categoria_tiene_sus_datos_cargados(c) );
            res
        }


        /// Dado un id de categoria, retorna su data
        /// 
        /// La informacion del la categoria se retornara en el siguiente formato: 
        /// 
        /// 0: id unico que tiene en el club
        /// 
        /// 1: costo_mensual_en_tokens
        /// 
        /// 2: actividades_accesibles_base
        /// 
        /// Posibles Error: CategoriaInvalida, CategoriaSinData
        #[ink(message)]
        pub fn get_data_categoria_datos(&self,id_categoria: u32) -> Result<(u32,u128,Vec<Actividad>),Error>{
            self.get_data_categoria_datos_priv(id_categoria)
        }
        fn get_data_categoria_datos_priv(&self,id_categoria: u32) -> Result<(u32,u128,Vec<Actividad>),Error>{
            if !self.existe_categoria_con_id(id_categoria) {return Err(Error::CategoriaInvalida)}
            if !self.categoria_tiene_sus_datos_cargados(id_categoria) {return Err(Error::CategoriaSinData)}
            
            let binding = self.get_categoria_datos(id_categoria);
            let Ok(data) = binding else {return Err(binding.err().unwrap())};

            Ok((data.id,data.costo_mensual_en_tokens,data.actividades_accesibles_base))
        }
        /// METODO PRIVADO 
        /// 
        /// Dado un id de categoria, retorna su la data
        /// 
        /// Posibles Error: CategoriaInvalida, CategoriaSinData
        fn get_categoria_datos(&self,id_categoria:u32) -> Result<DatosCategoria,Error>{
            if !self.existe_categoria_con_id(id_categoria) {return Err(Error::CategoriaInvalida)}
            if !self.categoria_tiene_sus_datos_cargados(id_categoria) {return Err(Error::CategoriaSinData)}
            
            Ok(self.categorias_data.get(id_categoria).unwrap().clone())
        }


    
    




        // ---------------------- SOBRE LOS PAGOS ---------------------------




        /// Dado un pago, retorna true si si el pago esta vencido
        /// 
        /// Un pago vencido es aquel que se pago luego de la fecha de vencimiento, o que no se pago y ya paso la fecha de vencimiento 
        /// 
        /// Posibles Error: PagoNoRegistrado
        #[ink(message)]
        pub fn pago_esta_vencido (&self,pago_id:u128) -> Result<bool,Error>{
            let pago_id = pago_id as usize;
            let pagos = self.get_pagos().unwrap();
            if pagos.len()<=pago_id {return Err(Error::PagoNoRegistrado);};

            let pago = &pagos[pago_id];
            
            if let Some(fecha_de_pago) = pago.fecha_de_pago.clone(){
                return Ok(fecha_de_pago > pago.fecha_de_vencimiento)
            }
            return Ok(LocalDateTime::now(self) > LocalDateTime::at(pago.fecha_de_vencimiento as i64));
        }


        /// Dado el id de un socio, se listan sus pagos. 
        /// 
        /// Se necesitan permisos
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
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, SocioNoRegistrado
        #[ink(message)]
        pub fn get_data_pagos_del_socio_con_id(&self, socio_id:u128)-> Result<Vec<(u128,u128,Option<Timestamp>,Timestamp,u128,bool)>,Error>{
            self.get_data_pagos_del_socio_con_id_priv(socio_id)
        }
        fn get_data_pagos_del_socio_con_id_priv(&self, socio_id:u128)-> Result<Vec<(u128,u128,Option<Timestamp>,Timestamp,u128,bool)>,Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes)}

            let binding = self.get_pagos_del_socio_con_id(socio_id);
            let Ok(data) = binding else {return Err(binding.err().unwrap())};

            Ok( data.iter().map(|d| (d.id,d.socio_id,d.fecha_de_pago,d.fecha_de_vencimiento,d.monto,d.tiene_bonificacion)).collect())
        }
        /// METODO PRIVADO
        /// 
        /// Dado el id de un socio, se listan sus pagos. 
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, SocioNoRegistrado
        fn get_pagos_del_socio_con_id(&self, socio_id:u128)->Result<Vec<Pago>,Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes)}
            if !self.existe_socio_con_id(socio_id) {return Err(Error::SocioNoRegistrado)}
            let pagos_de_un_socio = self.get_pagos().unwrap().into_iter().filter(|p|p.socio_id == socio_id).collect();
            Ok(pagos_de_un_socio)
        }
        

        /// Retorna todos datos de los pagos que se han realizado al Club
        /// 
        /// Se necesitan permisos
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
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes
        #[ink(message)]
        pub fn get_data_pagos(&self) -> Result<Vec<(u128,u128,Option<Timestamp>,Timestamp,u128,bool)>,Error>{
            self.get_data_pagos_priv()
        }
        fn get_data_pagos_priv(&self) -> Result<Vec<(u128,u128,Option<Timestamp>,Timestamp,u128,bool)>,Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes)}

            let binding = self.get_pagos();
            let Ok(data) = binding else {return Err(binding.err().unwrap())};

            Ok( data.iter().map(|d| (d.id,d.socio_id,d.fecha_de_pago,d.fecha_de_vencimiento,d.monto,d.tiene_bonificacion)).collect())
        }
        /// METODO PRIVADO
        /// 
        /// Retorna todos los pagos que se han realizado al Club
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes
        fn get_pagos(&self) -> Result<Vec<Pago>,Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes)}
            let todos_los_pagos = self.pagos.get_or_default(); //CONSULTA: tambien podria ser self.pagos.get();
            Ok(todos_los_pagos)
        }



        /// Dado el id de un socio, retorna una lista de la data de los pagos del mes y anio indicados de ese socio
        /// 
        /// Se necesitan permisos
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
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, FechaInvalida
        #[ink(message)]
        pub fn get_data_pagos_del_mes_y_anio(&self, mes:i8, anio:i64) -> Result<Vec<(u128,u128,Option<Timestamp>,Timestamp,u128,bool)>,Error>{
            self.get_data_pagos_del_mes_y_anio_priv(mes,anio)
        } 
        fn get_data_pagos_del_mes_y_anio_priv(&self, mes:i8, anio:i64) -> Result<Vec<(u128,u128,Option<Timestamp>,Timestamp,u128,bool)>,Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes)}

            let binding = Self::construir_fecha_midnight(anio,mes,1);
            let Ok(mes_y_anio) = binding else {return Err(binding.err().unwrap())};

            let binding = self.get_pagos_del_mes_y_anio(mes_y_anio);
            let Ok(data) = binding else {return Err(binding.err().unwrap())};

            Ok( data.iter().map(|d| (d.id,d.socio_id,d.fecha_de_pago,d.fecha_de_vencimiento,d.monto,d.tiene_bonificacion)).collect())
        } 
        /// METODO PRIVADO
        /// 
        /// Retorna una lista de los pagos del mes y anio indicados
        /// 
        /// Si en la fecha no se realizaron pagos, se retornara una lista vacia
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes
        fn get_pagos_del_mes_y_anio(&self,mes_y_anio:LocalDateTime) -> Result<Vec<Pago>,Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes)}
            let pagos_del_mes_y_anio = self.get_pagos().unwrap().into_iter().filter(|p|
                LocalDateTime::at(p.fecha_de_vencimiento as i64).date().month() == mes_y_anio.date().month() &&
                LocalDateTime::at(p.fecha_de_vencimiento as i64).date().year() == mes_y_anio.date().year())      .collect();
            Ok(pagos_del_mes_y_anio)
        }




        


        /// Dado el id de un socio, devuelve la data del primer pago sin acreditar
        /// 
        /// Se necesitan permisos
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
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, SocioNoRegistrado, SocioNoPoseePagosSinAcreditar
        #[ink(message)]
        pub fn get_data_primer_pago_sin_acreditar_del_socio_con_id(&self, socio_id:u128) -> Result<(u128,u128,Option<Timestamp>,Timestamp,u128,bool),Error>{
            self.get_data_primer_pago_sin_acreditar_del_socio_con_id_priv(socio_id)
        } 
        fn get_data_primer_pago_sin_acreditar_del_socio_con_id_priv(&self, socio_id:u128) -> Result<(u128,u128,Option<Timestamp>,Timestamp,u128,bool),Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes)}
            if !self.existe_socio_con_id(socio_id) {return Err(Error::SocioNoRegistrado)}

            let binding = self.get_primer_pago_sin_acreditar_del_socio_con_id(socio_id);
            let Ok(pago) = binding else {return Err(binding.err().unwrap())};

            Ok((pago.id,pago.socio_id,pago.fecha_de_pago,pago.fecha_de_vencimiento,pago.monto,pago.tiene_bonificacion))
        } 
        /// METODO PRIVADO
        ///         
        /// Dado el id de un socio, devuelve el primer pago sin acreditar
        /// 
        /// Posibles Error: SocioNoRegistrado, SocioNoPoseePagosSinAcreditar
        fn get_primer_pago_sin_acreditar_del_socio_con_id(&self, socio_id:u128) -> Result<Pago,Error>{
            if !self.existe_socio_con_id(socio_id) {return Err(Error::SocioNoRegistrado)}
            let Some(pago) = self.get_primer_pago_sin_acreditar(socio_id) else {return Err(Error::SocioNoPoseePagosSinAcreditar)};
            Ok(pago)
        } 






        // ---------------------- SOBRE EL SOCIO ---------------------------


        /// Dado un dni retorna el id del socio, si es que existe
        #[ink(message)]
        pub fn get_id_socio_con_dni(&self, un_dni:u32)->Option<u128>{
            self.get_id_socio_con_dni_priv(un_dni)
        }
        fn get_id_socio_con_dni_priv(&self, un_dni:u32)->Option<u128>{
            let len = self.mapping_lens.socios;
            for i in 1..len+1{
                let socio = self.socios.get(i).unwrap();
                if socio.datos_personales.dni == un_dni {
                    return Some(socio.id);
                }
            }
            None
        }

        /// Dado un id, retorna la data del socio
        /// 
        /// Se necesitan permisos
        /// 
        /// La informacion se retornara en el siguiente formato: 
        /// 
        /// 0: nombre
        /// 
        /// 1: apellido
        /// 
        /// 2: dni
        /// 
        /// 3: id unico que tiene en el club
        /// 
        /// 4: categoria
        /// 
        /// Posibles Error: NoSePoseenLosPermisosSuficientes, SocioNoRegistrado
        #[ink(message)]
        pub fn get_data_socio_con_id(&self,socio_id:u128)->Result<(String,String,u32,u128,Categoria),Error>{
            self.get_data_socio_con_id_priv(socio_id)
        }
        fn get_data_socio_con_id_priv(&self,socio_id:u128)->Result<(String,String,u32,u128,Categoria),Error>{
            if !self.tiene_permiso() {return Err(Error::NoSePoseenLosPermisosSuficientes)}
            if !self.existe_socio_con_id(socio_id) {return Err(Error::SocioNoRegistrado)}
            
            let socio = self.get_socio_con_id(socio_id).unwrap();
            Ok((socio.datos_personales.nombre,socio.datos_personales.apellido,socio.datos_personales.dni,socio.id,socio.categoria))
        }
        /// METODO PRIVADO
        /// 
        /// Dado un id devuelve el socio, si es que existe
        fn get_socio_con_id(&self,socio_id:u128)->Option<Socio>{
            self.socios.get(socio_id)
        }

        /// Retorna true si el socio tiene permitida la asistencia a la actividad, false en caso contrario
        /// 
        /// Posibles Error: ActividadInvalida, SocioNoRegistrado, CategoriaSinData
        #[ink(message)]
        pub fn socio_tiene_permitida_la_asistencia_a(&self, socio_id:u128,id_actividad: u32) -> Result<bool,Error>{
            self.socio_tiene_permitida_la_asistencia_a_priv(socio_id,id_actividad)
        }
        fn socio_tiene_permitida_la_asistencia_a_priv(&self, socio_id:u128,id_actividad: u32) -> Result<bool,Error>{
            if !self.existe_actividad_con_id(id_actividad) {return Err(Error::ActividadInvalida)}
            if !self.existe_socio_con_id(socio_id) {return Err(Error::SocioNoRegistrado)}

            let categoria = self.get_socio_con_id(socio_id).unwrap().categoria;
            
            match categoria.clone(){
                Categoria::B{deporte_seleccionado_por_el_usuario} => {if deporte_seleccionado_por_el_usuario as u32 == id_actividad {return Ok(true);}}
                _ => {}
            }

            let binding = self.get_categoria_datos(categoria.discriminant());
            let Ok(categoria_data) = binding else {return Err(binding.err().unwrap())};

            let binding = categoria_data.actividades_accesibles_base;
            let res = binding.iter().find(|a|a.clone().clone() as u32 == id_actividad);

            Ok(res.is_some())
        }



        /// Dado un ID, retorna true si existe un socio con ese id 
        #[ink(message)]
        pub fn existe_socio_con_id(&self, socio_id:u128) -> bool{
            self.existe_socio_con_id_priv(socio_id)
        }
        fn existe_socio_con_id_priv(&self, socio_id:u128) -> bool{
            self.socios.contains(socio_id)
        }

        /// Retorna true si el socio con el dni pasado por parametro es un socio del club, false en caso contrario 
        #[ink(message)]
        pub fn existe_socio_dni(&self,un_dni:u32)->bool{
            self.existe_socio_dni_priv(un_dni)
        }
        fn existe_socio_dni_priv(&self,un_dni:u32)->bool{
            self.get_id_socio_con_dni(un_dni).is_some()
        }
    }









/// Module ink_env::test -> https://paritytech.github.io/ink/ink_env/test/index.html
/// Examples -> https://github.com/paritytech/ink-examples/blob/main/erc20/lib.rs
    #[cfg(test)]
    mod tests {

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
            let categoria_data_a = club.get_data_categoria_datos(Categoria::A.discriminant());
            
            assert!(categoria_data_a.is_ok());
            let categoria_data_a = categoria_data_a.unwrap();
            assert_eq!((categoria_data_a.2),vec_de_actividades_a);
            assert_eq!(categoria_data_a.0,Categoria::A.discriminant());
            assert_eq!(categoria_data_a.1,5000);


            // se crea y carga la categoria b
            let mut vec_de_actividades_b=Vec::new();
            vec_de_actividades_b.push(Actividad::Gimnasio);
            // la actividad puede ser cualquiera, no se va a guargar, es solo para obtener el indice
            assert!(club.cargar_data_categoria(Categoria::B { deporte_seleccionado_por_el_usuario:Actividad::default() }, 3000, vec_de_actividades_b.clone()).is_ok());

            // chequeamos si los datos se guardaron correctamente en la categoria b
            let categoria_data_b = club.get_data_categoria_datos(Categoria::B { deporte_seleccionado_por_el_usuario:Actividad::default() }.discriminant());
                        
            assert!(categoria_data_b.is_ok());
            let categoria_data_b = categoria_data_b.unwrap();
            assert_eq!((categoria_data_b.2),vec_de_actividades_b);
            assert_eq!(categoria_data_b.0,Categoria::B { deporte_seleccionado_por_el_usuario:Actividad::default()}.discriminant());
            assert_eq!(categoria_data_b.1,3000);


            // se crea y carga la categoria c
            let mut vec_de_actividades_c=Vec::new();
            vec_de_actividades_c.push(Actividad::Gimnasio);
            assert!(club.cargar_data_categoria(Categoria::C,2000,vec_de_actividades_c.clone()).is_ok());

            // chequeamos si los datos se guardaron correctamente en la categoria c
            let categoria_data_c = club.get_data_categoria_datos(Categoria::C.discriminant());
            
            assert!(categoria_data_c.is_ok());
            let categoria_data_c = categoria_data_c.unwrap();
            assert_eq!((categoria_data_c.2),vec_de_actividades_c);
            assert_eq!(categoria_data_c.0,Categoria::C.discriminant());
            assert_eq!(categoria_data_c.1,2000);

            assert!(club.todas_las_categorias_tienen_sus_datas_cargadas());
        }

        //-------------------------- tests ----------------------

        #[ink::test]
        fn test_get_ids_categorias() {
            let club = crear_club_sem_rust();
            
            let mut ids = Vec::new();
            for i in 0..club.cant_categorias() as u32{
                ids.push(i);
            }
            assert_eq!(ids,club.get_ids_categorias());
        }
        #[ink::test]
        fn test_get_ids_actividades() {
            let club = crear_club_sem_rust();

            let mut ids = Vec::new();
            for i in 0..club.cant_actividades() as u32{
                ids.push(i);
            }
            assert_eq!(ids,club.get_ids_actividades());
        }
        
        #[ink::test]
        fn test_existe_actividad_con_id () {
            let club = crear_club_sem_rust();
            assert!(club.existe_actividad_con_id(Actividad::Basquet as u32));
            assert!(club.existe_actividad_con_id(Actividad::Natacion as u32));
            assert!(club.existe_actividad_con_id(Actividad::Futbol as u32));
            assert!(!club.existe_actividad_con_id(800));
        }
        #[ink::test]
        fn test_existe_categoria_con_id (){
            let club = crear_club_sem_rust();
            assert!(club.existe_categoria_con_id(Categoria::A.discriminant()));
            assert!(club.existe_categoria_con_id(Categoria::B{deporte_seleccionado_por_el_usuario: Actividad::default()}.discriminant()));
            assert!(club.existe_categoria_con_id(Categoria::C.discriminant()));
            assert!(!club.existe_categoria_con_id(800));
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
        fn test_set_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento(){
            let mut club = crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert!(club.set_cant_pagos_consecutivos_sin_atrasos_necesarios_para_descuento(5).is_ok());
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
        fn test_get_data_pagos(){ 
            let mut club=crear_club_sem_rust();
            crear_y_cargar_categorias(&mut club);
            let dni1 = 90;
            let dni2=30;
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni1, Categoria::B { deporte_seleccionado_por_el_usuario: (Actividad::Futbol) }).is_ok());
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Perri".to_string(),dni2, Categoria::C).is_ok());
            let id1 = club.get_id_socio_con_dni(dni1).unwrap();
            let id2 = club.get_id_socio_con_dni(dni2).unwrap();
            let pagos_totales=club.get_data_pagos();
            let mut pagos_totales_manual=Vec::new();
            pagos_totales_manual.push(club.get_data_pagos_del_socio_con_id(id1).unwrap().pop().unwrap());
            pagos_totales_manual.push(club.get_data_pagos_del_socio_con_id(id2).unwrap().pop().unwrap());
            assert_eq!(pagos_totales.unwrap(),pagos_totales_manual);
        }
        #[ink::test]
        fn test_get_data_pagos_del_socio_con_id(){
            let mut club=crear_club_sem_rust();
            crear_y_cargar_categorias(&mut club);
            let dni1 = 90;
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni1, Categoria::B { deporte_seleccionado_por_el_usuario: (Actividad::Futbol) }).is_ok());
            let id1 = club.get_id_socio_con_dni(dni1).unwrap();
            
            let pago_de_charlie=club.get_data_pagos_del_socio_con_id(id1).unwrap();
            let pago_de_charlie_manual=club.get_data_pagos().unwrap();
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
        fn test_get_data_pagos_del_mes_y_anio(){
            let mut club=crear_club_sem_rust();
            crear_y_cargar_categorias(&mut club);

            let mes_y_anio_de_venicimiento_buscados = LocalDateTime::new(
                LocalDate::ymd(2021,Month::October,10).unwrap(),  
                LocalTime::midnight() 
            );
            let fecha_de_vencimiento_no_buscada =  LocalDateTime::now(&club);

            let dni1 = 90;
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni1, Categoria::A).is_ok());
            let id1 = club.get_id_socio_con_dni(dni1).unwrap();

            assert!((club.crear_cuota_para_socio(id1,mes_y_anio_de_venicimiento_buscados)).is_ok());
            assert!((club.crear_cuota_para_socio(id1,fecha_de_vencimiento_no_buscada)).is_ok());
            
            let dni2 = 57;
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni2, Categoria::B { deporte_seleccionado_por_el_usuario: (Actividad::Futbol) }).is_ok());
            let id2 = club.get_id_socio_con_dni(dni2).unwrap();
            assert!((club.crear_cuota_para_socio(id2,mes_y_anio_de_venicimiento_buscados)).is_ok());
            assert!((club.crear_cuota_para_socio(id2,fecha_de_vencimiento_no_buscada)).is_ok());

            let pagos_del_club=club.get_pagos().unwrap();
            let pagos_anio_y_mes = club.get_data_pagos_del_mes_y_anio(10,2021).unwrap();
            assert_eq!(pagos_anio_y_mes, 
                                        pagos_del_club.into_iter().filter(|p|
                                            LocalDateTime::at(p.fecha_de_vencimiento as i64).date().month() == mes_y_anio_de_venicimiento_buscados.date().month() &&
                                            LocalDateTime::at(p.fecha_de_vencimiento as i64).date().year() == mes_y_anio_de_venicimiento_buscados.date().year()) 
                                                .map(|d| (d.id,d.socio_id,d.fecha_de_pago,d.fecha_de_vencimiento,d.monto,d.tiene_bonificacion)).collect::<Vec<(u128, u128, Option<u64>, u64, u128, bool)>>()
                                        );
        }
        #[ink::test]
        fn test_get_data_primer_pago_sin_acreditar_del_socio_con_id_y_registrar_nuevo_pago(){
            let mut club = crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            crear_y_cargar_categorias(&mut club);
            
            let dni1 = 75;
            // al registrar un nuevo socio se le crea una cuota sin pagar
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni1, Categoria::B { deporte_seleccionado_por_el_usuario: (Actividad::Futbol) }).is_ok());
            let id1 = club.get_id_socio_con_dni(dni1).unwrap();
            
            // obtengo el primer pago sin pagar. Ya que no paso un mes desde la creacion del club, no se creara otra cuota
            let pago = club.get_data_primer_pago_sin_acreditar_del_socio_con_id(id1);
            assert!(pago.is_ok());
            let pago = pago.unwrap();
            // pagos_del_usuario_originales contiene el primer pago sin pagar.
            let pagos_del_usuario_originales = vec![pago.clone()];
            assert_eq!(club.get_data_pagos_del_socio_con_id(id1).unwrap(),pagos_del_usuario_originales);
            assert!(club.registrar_nuevo_pago(id1, pago.4).is_ok());
            // Si registrar_nuevo_pago() funciona correctamente, luego de ejecutarlo no deverian ser iguales
            assert_ne!(club.get_data_pagos_del_socio_con_id(id1).unwrap(),pagos_del_usuario_originales);

            // no tendria que tener pagos sin pagar
            assert_eq!(club.get_data_primer_pago_sin_acreditar_del_socio_con_id(id1).err().unwrap(),Error::SocioNoPoseePagosSinAcreditar);

            let pagos =club.get_data_pagos_del_socio_con_id(id1);
            assert!(pagos.is_ok());
            let pagos = pagos.unwrap();
            // hay un solo pago de ese usuario
            assert_eq!(pagos.len(),1);
            let pago = &pagos[0];
            // el pago tendria que tener fecha de pago
            assert!(pago.2.is_some());
        }

        #[ink::test]
        fn test_crear_cuota_para_socio(){
            let mut club = crear_club_sem_rust();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            crear_y_cargar_categorias(&mut club);
            let dni = 8;
            club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni, Categoria::C);
            let id1 = club.get_id_socio_con_dni(dni).unwrap();

            let fecha_de_vencimiento =  LocalDateTime::now(&club).to_instant().seconds() + 604800;
            assert!((club.crear_cuota_para_socio(id1,LocalDateTime::at(fecha_de_vencimiento as i64))).is_ok());
            
            let pagos =club.get_pagos_del_socio_con_id(id1);
            assert!(pagos.is_ok());
            let pagos = pagos.unwrap();
            // hay 2 pagos de ese usuario
            assert_eq!(pagos.len(),2);
            let pago0 = &pagos[0];
            let pago1 = &pagos[1];

            assert_eq!(pago1.id,1);
            assert_eq!(pago1.socio_id,1);
            assert!(pago1.fecha_de_pago.is_none());
            assert_eq!(pago1.fecha_de_vencimiento,fecha_de_vencimiento as u64);
            assert!(!pago1.tiene_bonificacion);
            let monto = club.get_categoria_datos(Categoria::C.discriminant()).unwrap().costo_mensual_en_tokens;
            assert_eq!(pago1.monto,monto);
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
            let socio_id = club.get_id_socio_con_dni(dni).unwrap();
            
            assert_eq!(club.pagos.get_or_default().len(),1);

            // no tiene bonificacion todavia
            assert!(!club.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id).unwrap());

            // paga la primer cuota al dia
            let pago = club.get_primer_pago_sin_acreditar_del_socio_con_id(socio_id).unwrap();
            assert!(!pago.tiene_bonificacion);
            assert!(!club.pago_esta_vencido(pago.id).unwrap());
            assert!(club.registrar_nuevo_pago(socio_id, pago.monto).is_ok());
            assert!(!club.pago_esta_vencido(pago.id).unwrap());

            // no tiene bonificacion todavia
            assert!(!club.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id).unwrap());

            // se le agregan 2 cuotas y las paga al dia
            let fecha_de_vencimiento =  LocalDateTime::now(&club).to_instant().seconds() + 604800;
            assert!((club.crear_cuota_para_socio(socio_id,LocalDateTime::at(fecha_de_vencimiento as i64))).is_ok());
            assert_eq!(club.pagos.get_or_default().len(),2);
            let pago = club.get_primer_pago_sin_acreditar_del_socio_con_id(socio_id).unwrap();
            assert!(!pago.tiene_bonificacion);
            //assert!(!club.pago_esta_vencido(pago.id).unwrap());
            assert!(club.registrar_nuevo_pago(socio_id, pago.monto).is_ok());
            assert!(!club.pago_esta_vencido(pago.id).unwrap());

            // no tiene bonificacion todavia
            assert!(!club.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id).unwrap());

            let fecha_de_vencimiento =  LocalDateTime::now(&club).to_instant().seconds() + 604800;
            assert!((club.crear_cuota_para_socio(socio_id,LocalDateTime::at(fecha_de_vencimiento as i64))).is_ok());
            assert_eq!(club.pagos.get_or_default().len(),3);
            let pago = club.get_primer_pago_sin_acreditar_del_socio_con_id(socio_id).unwrap();
            assert!(!pago.tiene_bonificacion);
            assert!(!club.pago_esta_vencido(pago.id).unwrap());
            assert!(club.registrar_nuevo_pago(socio_id, pago.monto).is_ok());
            assert!(!club.pago_esta_vencido(pago.id).unwrap());

            // Ya que pago 3 cuotas sin que alguna se venciese, le corresponde bonificacion
            assert!(club.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id).unwrap());
            
            // se le agrega otra cuota (esta tiene bonificacion) y la paga al dia
            let fecha_de_vencimiento =  LocalDateTime::now(&club).to_instant().seconds() + 604800;
            assert!((club.crear_cuota_para_socio(socio_id,LocalDateTime::at(fecha_de_vencimiento as i64))).is_ok());
            assert_eq!(club.pagos.get_or_default().len(),4);
            let pago = club.get_primer_pago_sin_acreditar_del_socio_con_id(socio_id).unwrap();
            assert!(pago.tiene_bonificacion);
            assert!(!club.pago_esta_vencido(pago.id).unwrap());
            assert!(club.registrar_nuevo_pago(socio_id, pago.monto).is_ok());
            assert!(!club.pago_esta_vencido(pago.id).unwrap());

            // aunque haya pagado 4 cuotas seguidas con al dia, ya que obtuvo bonificacion recientemente, no le corresponde bonificacion
            assert!(!club.socio_cumple_las_condiciones_para_obtener_la_bonificacion(socio_id).unwrap());

        }
        #[ink::test]
        fn test_socio_tiene_permitida_la_asistencia_a_con_categoria_a(){
            let mut club=crear_club_sem_rust();
            crear_y_cargar_categorias(&mut club);
            let dni = 47;
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni, Categoria::A).is_ok());
            let socio_id = club.get_id_socio_con_dni(dni).unwrap();
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
            let socio_id = club.get_id_socio_con_dni(dni).unwrap();
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
            let socio_id = club.get_id_socio_con_dni(dni).unwrap();
            assert!(club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Gimnasio as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Futbol as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Basquet as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Natacion as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Hockey as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Paddle as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Rugby as u32).unwrap());
            assert!(!club.socio_tiene_permitida_la_asistencia_a(socio_id,Actividad::Tenis as u32).unwrap());
        }
        #[ink::test]
        fn test_get_data_socio_con_id(){ 
            let mut club=crear_club_sem_rust();
            crear_y_cargar_categorias(&mut club);
            let dni1 = 90;
            let dni2=30;
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni1, Categoria::A).is_ok());
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Perri".to_string(),dni2, Categoria::C).is_ok());
            let id1 = club.get_id_socio_con_dni(dni1).unwrap();
            let id2 = club.get_id_socio_con_dni(dni2).unwrap();
            
            let data_socio1=club.get_data_socio_con_id(id1);
            assert!(data_socio1.is_ok());
            let data_socio1 = data_socio1.unwrap();

            let data_socio2=club.get_data_socio_con_id(id2);
            assert!(data_socio2.is_ok());
            let data_socio2 = data_socio2.unwrap();

            assert_eq!(data_socio1,("charlie".to_string(), "Ricciardi".to_string(), dni1, id1, Categoria::A));
            assert_eq!(data_socio2,("charlie".to_string(), "Perri".to_string(), dni2, id2, Categoria::C));
        }
        #[ink::test]
        fn test_construir_fecha_midnight(){
            assert!(ClubSemRust::construir_fecha_midnight(2000, 90, 8).is_err());
            assert!(ClubSemRust::construir_fecha_midnight(2000, 9, 80).is_err());

            assert_eq!(ClubSemRust::construir_fecha_midnight(2000, 4, 1).unwrap(), 
                                                                        LocalDateTime::new(
                                                                            LocalDate::ymd(2000,Month::from_one(4).unwrap(),1).unwrap(),  
                                                                            LocalTime::midnight() 
                                                                                )
                        );
        }
        #[ink::test]
        fn test_get_actividad_id (){
            let mut club=crear_club_sem_rust();
            assert_eq!(club.get_actividad_id(Actividad::Natacion),Actividad::Natacion as u32)
        }
        #[ink::test]
        fn test_get_categoria_id  (){
            let mut club=crear_club_sem_rust();
            assert_eq!(club.get_categoria_id (Categoria::C),Categoria::C.discriminant())
        }
        #[ink::test]
        fn test_registrar_nuevo_pago_ajecuta_actualizacion_mensual_y_da_ok  (){
            // para corroborar esto hay que seguir el html
            // lo que se hace es simil "test_actualizacion_mensual", como para que al ejecutar actualizacion mensual en registrar_nuevo_pago, retorne Ok
            let mut club=crear_club_sem_rust();
            crear_y_cargar_categorias(&mut club);
            club.fecha_de_la_ultima_actualizacion-= 2678400;

            let dni = 75;
            assert!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),dni, Categoria::B { deporte_seleccionado_por_el_usuario: (Actividad::Futbol) }).is_ok());
            let socio_id = club.get_id_socio_con_dni(dni).unwrap();

            let fecha_de_vencimiento =  LocalDateTime::now(&club).to_instant().seconds() + 604800;
            assert!((club.crear_cuota_para_socio(socio_id,LocalDateTime::at(fecha_de_vencimiento as i64))).is_ok());
            let pago = club.get_primer_pago_sin_acreditar_del_socio_con_id(socio_id).unwrap();
            assert!(club.registrar_nuevo_pago(socio_id, pago.monto).is_ok());
        }
        #[ink::test]
        fn test_actualizacion_mensual_falla_en_crear_cuota_para_socio() {
            let mut club=crear_club_sem_rust();
            assert_eq!(club.registrar_nuevo_socio("charlie".to_string(),"Ricciardi".to_string(),50, Categoria::C).err().unwrap(),Error::CategoriaSinData);

            // Si no se carga la data de la categoria no se permite agregar un nuevo socio, acualizacion mensual retorna Ok(), pero sin crear ninguna cuota
            club.fecha_de_la_ultima_actualizacion-= 2678400;
            assert!(club.actualizacion_mensual().is_ok());
            assert_eq!(club.get_pagos().unwrap().len(),0);

            // creo un socio manualmente, sin cargar las datas de las categorias (esto es imposible de hacerlo desde afuera)

            let info_personal_del_socio=DatosPersonalesSocio{nombre:"charlie".to_string(),apellido:"Perri".to_string(),dni:90};
            let socio=Socio{id:club.nueva_id(TipoId::Socio),categoria:Categoria::C,datos_personales:info_personal_del_socio};
            club.socios.insert(socio.id, &socio);

            // ya que la categoria del socio no tiene su data cargada, al llamar crear_cuota_para_socio dentro de actualizacion_mensual, saltara el error
            club.fecha_de_la_ultima_actualizacion-= 2678400;
            assert_eq!(club.actualizacion_mensual().err().unwrap(),Error::CategoriaSinData)
        }

    

    }

}
