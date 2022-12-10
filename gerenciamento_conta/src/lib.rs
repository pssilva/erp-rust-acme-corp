
pub fn health_check() -> bool {

    println!("");
    println!("=============================================");
    println!("Módulo :: Gestão de Contas :: Não implementado!!");
    println!("=============================================");
    println!("");
    
    false
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
