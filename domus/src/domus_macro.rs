

macro_rules! define_space_field_type {
    (
        $field_name:ident,
        Space 
    ) => {
        paste! {
            [<$field_name:camel>]
        }
    };    
    (
        $field_name:ident,
        $field_type:ty 
    ) => {
        $field_type
    };
}


macro_rules! define_space {
    (
        $name:ident, 
        Space { 
            name: $display_name:expr
            $(, $field_name:ident: $field_type:ident { $($props:tt)+ })*
            $(,)?
        }
    ) => {
        paste! {

            #[derive(Debug)]
            #[allow(unused)]
            struct [<$name:camel>] {
                name: &'static str,
                $(pub $field_name: define_space_field_type!($field_name, $field_type),)*
            }

            impl Space for [<$name:camel>] {
                fn name(&self) -> &str {
                    &self.name
                }
            }

            impl LifeCycle for [<$name:camel>] {
                async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
                    log::info!("Initializing {}", self.name);
                    $(let _ = &self.$field_name.init().await?;)*
                    Ok(())
                }

                async fn dispose(&self) -> Result<(), Box<dyn std::error::Error>> {
                    log::info!("Disposing {}", self.name);
                    $(let _ = &self.$field_name.dispose().await?;)*
                    Ok(())
                }
            }
        }   
        $(define_space!($field_name, $field_type { $($props)* });)*
        
    };
    ($name:ident, $device_type:ident $($rest:tt)*
    ) => {
        // Device definition doesn't generate any typess
    };
}

macro_rules! init_space_field_value {
    (
        $name:ident:
        Space { 
            name: $display_name:expr
            $(, $field_name:ident: $field_type:ident { $($subspace:tt)* })*
            $(,)?
        }
    ) => {
        paste! {
            [<$name:camel>] {
                name: $display_name,
                $($field_name: init_space_field_value!($field_name: $field_type { $($subspace)* }),)*
            }
        }
    };    
    (
        $name:ident:
        $device_type:ident { $($props:tt)* }
    ) => {
        $device_type {
            $($props)*
        }
    };
}

macro_rules! domus {
    (
        name: $name:expr
        $(, $field_name:ident: $field_type:ident { $($subspace:tt)+ })*
        $(,)?
    ) => {
        paste! {
            {
                #[derive(Debug)]
                #[allow(unused)]
                struct Domus {
                    name: String,
                    $($field_name: define_space_field_type!($field_name, $field_type),)*
                }

                impl Space for Domus {
                    fn name(&self) -> &str {
                        &self.name
                    }

                }

                impl LifeCycle for Domus {
                    async fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
                        log::info!("Initializing {}", self.name);
                        $(let _ = &self.$field_name.init().await?;)*
                        Ok(())
                    }

                    async fn dispose(&self) -> Result<(), Box<dyn std::error::Error>> {
                        log::info!("Disposing {}", self.name);
                        $(let _ = &self.$field_name.dispose().await?;)*
                        Ok(())
                    }
                }

                $(
                    define_space!($field_name, $field_type { $($subspace)* });
                )*

                Domus {
                    name: $name.to_string(),
                    $($field_name: init_space_field_value!($field_name: $field_type { $($subspace)* }),)*
                }
            }
        }
    };
}