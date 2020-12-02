mod c_404;

pub fn all() -> Vec<rocket::Catcher> {
    catchers![self::c_404::view,]
}
