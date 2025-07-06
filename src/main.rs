use serde_json::Value as json_value;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pessoa{
    pub id: u64,
    pub nome: String,
    pub idade: u8,
    pub email: String,
    pub profissao: String,
    pub cidade: String,
    pub salario: f64,
    pub status: bool,
    pub telefone: String,
    pub endereco: Endereco,
}

#[derive(Deserialize, Debug)]
pub struct Endereco {
    pub rua: String,
    pub cep: String,
    pub bairro: String,
}


fn main() {
    let content = std::include_str!("../src/serde_json/data.json");
    let json_converter: json_value = serde_json::from_str(content).unwrap();

    let nome = &json_converter["nome"];
    let rua = &json_converter["endereco"]["rua"].as_str().unwrap();
    let cep = &json_converter["endereco"]["cep"].as_str().unwrap();
    let bairro = &json_converter["endereco"]["bairro"].as_str().unwrap();

    dbg!(nome,rua,cep,bairro);


    //Usando Structs e Deserialize
    let pessoa: Pessoa = serde_json::from_str(content).unwrap();
    dbg!(pessoa);

    
}
