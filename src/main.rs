fn main(){

}

struct Product {
    id: u32,
    name: String,
    price: u32,
    stock: u32,
}

impl Product {
    fn new_project(id: u32, name: &str, price: u32, stock:u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            price, 
            stock,
        }
    }

    fn decs_stock(&mut self, units: u32) -> Option<u32>{
        self.stock = self.stock.checked_sub(units)?;
        Some(self.stock)
    }

    fn price_update(&mut self, new_price:u32) {
        self.price = new_price;
    }
}

enum Status {
    Pendiente, 
    Enviado,
    Entregado,
    Cancelado(String),
}

fn show_status (pedido: Status){
    match  pedido {
        Status::Pendiente => println!("Pedido pendiente de enviar"),
        Status::Enviado => println!("Pedido enviado"),
        Status::Entregado => println!("Pedido entregado"),
        Status::Cancelado(mov) => println!("Pedido cancelado por {}", mov),
    }
}
