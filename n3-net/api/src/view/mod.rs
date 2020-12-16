pub mod catchers;

macro_rules! impl_routes {
    [
        ref=[$($name_r:ident),*,],
        mut=[$($name_m:ident),*,],
    ] => {
        $(
            mod $name_r;
        )*
        $(
            mod $name_m;
        )*

        pub mod routes {
            pub fn all() -> Vec<rocket::Route> {
                routes![
                    $(
                        super::$name_r::insert,
                        super::$name_r::get,
                        super::$name_r::get_all,
                        super::$name_r::delete,
                    )*
                    $(
                        super::$name_m::insert,
                        super::$name_m::get,
                        super::$name_m::get_all,
                        super::$name_m::update,
                        super::$name_m::delete,
                    )*
                ]
            }
        }
    };
}

impl_routes![
    ref=[
        works,
    ],
    mut=[
        machines,
    ],
];
