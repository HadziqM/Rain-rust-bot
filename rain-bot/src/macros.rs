#[macro_export]
macro_rules! command_reg {
    ( $( $x:expr ),* ) => {

        pub fn reg_command() -> HashMap<String, Box<dyn CommandInteractionTrait>> {
            let mut _y = HashMap::new();
            $(
                _y.insert($x.name(), Box::new($x) as Box<dyn CommandInteractionTrait>);
            )*
            _y
        }
    };
}
#[macro_export]
macro_rules! button_reg {
    ( $( $x:expr ),* ) => {

        pub fn reg_button() -> HashMap<String, Box<dyn ButtonInteractionTrait>> {
            let mut _y = HashMap::new();
            $(
                _y.insert($x.name(), Box::new($x) as Box<dyn ButtonInteractionTrait>);
            )*
            _y
        }
    };
}
#[macro_export]
macro_rules! modal_reg {
    ( $( $x:expr ),* ) => {

        pub fn reg_modal() -> HashMap<String, Box<dyn ModalInteractionTrait>> {
            let mut _y = HashMap::new();
            $(
                _y.insert($x.name(), Box::new($x) as Box<dyn ModalInteractionTrait>);
            )*
            _y
        }
    };
}

#[macro_export]
macro_rules! reg {
    ($name:ident,$trait:ident ,$($x:expr),*) => {
        pub fn $name() -> HashMap<String, Box<dyn $trait>> {
            let mut _x = HashMap::new();
            $(
                _x.extend($x);
            )*
            _x
        }
    };
}
