use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct Celular {
    pub modelo: String,
    pub precio: u64,
    pub stock: u16,
    pub disponible: bool,
}

#[derive(Debug)]
pub struct Tienda {
    pub owner: String,
    pub nombre: String,
    pub celulares: Vec<Celular>,
}

impl Tienda {

    pub fn crear_tienda(owner: String, nombre: String) -> Self {
        Self {
            owner,
            nombre,
            celulares: Vec::new(),
        }
    }

    pub fn registrar_celular(&mut self, modelo: String, precio: u64, stock: u16) {
        let celular = Celular {
            modelo,
            precio,
            stock,
            disponible: true,
        };

        self.celulares.push(celular);
    }

    pub fn eliminar_celular(&mut self, modelo: String) {
        self.celulares.retain(|c| c.modelo != modelo);
    }

    pub fn actualizar_stock(&mut self, modelo: String, nuevo_stock: u16) {
        for celular in &mut self.celulares {
            if celular.modelo == modelo {
                celular.stock = nuevo_stock;
            }
        }
    }

    pub fn alternar_disponibilidad(&mut self, modelo: String) {
        for celular in &mut self.celulares {
            if celular.modelo == modelo {
                celular.disponible = !celular.disponible;
            }
        }
    }

    pub fn ver_inventario(&self) {
        for celular in &self.celulares {
            println!("{:?}", celular);
        }
    }
}