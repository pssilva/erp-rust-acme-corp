
pub mod controllers;
pub mod models;
pub mod views;

use crate::controllers::*;

pub fn health_check() -> bool {
    
    controller::health_check();
    println!("");
    println!("=============================================");
    println!("Módulo :: Gestão de Contas :: Não implementado!!");
    println!("=============================================");
    println!("");
    
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_health_check_then_return_true() {
        let result = health_check();
        assert_eq!(result, true);
    }
}
