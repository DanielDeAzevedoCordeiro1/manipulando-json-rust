use serde_json::Value as json_value;

fn main() {
    let content = std::include_str!("../src/serde_json/data.json");

    let json_converter: json_value = serde_json::from_str(content).unwrap();
    
    let nome = &json_converter["nome"];
    let rua = &json_converter["endereco"]["rua"].as_str().unwrap();
    let cep = &json_converter["endereco"]["cep"].as_str().unwrap();
    let bairro = &json_converter["endereco"]["bairro"].as_str().unwrap();

    dbg!(nome,rua,cep,bairro);
}
