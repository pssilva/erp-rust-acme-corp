
pub mod controller {
    use crate::{models::*, views::*};


    pub fn health_check() -> bool {

        view::health_check();
        model::health_check();

        true
    }
}


