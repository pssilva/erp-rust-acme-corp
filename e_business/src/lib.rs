
pub fn health_check() -> bool {
    
    println!("");
    println!("=============================================");
    println!("Módulo :: E-Business :: Não implementado!!");
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
