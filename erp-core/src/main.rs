
pub mod controllers;
pub mod models;
pub mod views;

use crate::controllers::*;

use std::io;
use gerenciamento_conta;
use gestao_financeira;
use gestao_fabricacao;
use gerenciamento_producao;
use gestao_transporte;
use gerenciamento_vendas_distribuicao;
use gestao_recursos_humanos;
use gestao_cadeia_abastecimento;
use gestao_relacionamento_cliente;
use e_business;



fn main() {
    controller::health_check();
    
    println!("Hello, ERP Rust ACME Corp (erp-rust-acme-corp)!");

    let _rst = health_check_all();

}

fn health_check_all() -> Result<bool, io::Error> {

    let mut result = health_check();

    if !result {
        panic!("Módulo :: EPR Core :: Não saudável!!");
    }

    result = gerenciamento_conta::health_check();
    if !result {
        println!("");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("gerenciamento_conta :: Não saudável!!");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("");
    }

    result = gestao_financeira::health_check();
    if !result {
        println!("");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("gestao_financeira :: Não saudável!!");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("");
    }
    
    result = gestao_fabricacao::health_check();
    if !result {
        println!("");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("gestao_fabricacao :: Não saudável!!");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("");
    }
    
    result = gerenciamento_producao::health_check();
    if !result {
        println!("");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("gerenciamento_producao :: Não saudável!!");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("");
    }
    
    result = gestao_transporte::health_check();
    if !result {
        println!("");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("gestao_transporte :: Não saudável!!");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("");
    }
    
    result = gerenciamento_vendas_distribuicao::health_check();
    if !result {
        println!("");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("gerenciamento_vendas_distribuicao :: Não saudável!!");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("");
    }
    
    result = gestao_recursos_humanos::health_check();
    if !result {
        println!("");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("gestao_recursos_humanos :: Não saudável!!");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("");
    }
    
    result = gestao_cadeia_abastecimento::health_check();
    if !result {
        println!("");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("gestao_cadeia_abastecimento :: Não saudável!!");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("");
    }
    
    result = gestao_relacionamento_cliente::health_check();
    if !result {
        println!("");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("gestao_relacionamento_cliente :: Não saudável!!");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("");
    }
    
    result = e_business::health_check();
    if !result {
        println!("");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("e_business :: Não saudável!!");
        println!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXx");
        println!("");
    }

    Ok(result)  

}


pub fn health_check() -> bool {
    
    println!("");
    println!("=============================================");
    println!("Módulo :: EPR Core :: Não implementado!!");
    println!("=============================================");
    println!("");
    
    true
}