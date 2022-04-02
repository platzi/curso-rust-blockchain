// Contador incrementar, decrementar, obtener el estado, resetar a 0

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contador {
    valor: i8,
}

#[near_bindgen]
impl Contador {
    // obtener contador
    pub fn get_num(&self) -> i8 {
        return self.valor;
    }

    //Incrementar contador
    pub fn incrementar(&mut self) {
        self.valor += 1;
        let log_message = format!("Incrementado el contador a {}", self.valor);

        env::log(log_message.as_bytes());
        despues_de_que_cambie();
    }    
    
    // Decrementar
    pub fn decrementar(&mut self) {
        self.valor -= 1;
        let log_message = format!("Decrementado el contador a {}", self.valor);

        env::log(log_message.as_bytes());
        despues_de_que_cambie();
    }    

    // Resetear
    pub fn reset(&mut self) {
        self.valor = 0;
        let log_message = format!("Se ha reseteado el contador {}", self.valor);

        env::log(log_message.as_bytes());
        despues_de_que_cambie();
    }    
}

fn despues_de_que_cambie() {
    env::log("El valor ha cambiado".as_bytes());
}

// Pruebas unitarias

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext{
        VMContext{
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0,1,2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn incrementar() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Contador{ valor: 0};
        contract.incrementar();

        println!("Valor despues del incremento: {}", contract.valor);

        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrementar() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Contador{ valor: 0};
        contract.decrementar();

        println!("Valor despues del incremento: {}", contract.valor);

        assert_eq!(-1, contract.get_num());
    }


    #[test]
    fn reset() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Contador{ valor: 10};

        assert_eq!(10, contract.get_num());
        println!("Valor Antes del reset: {}", contract.valor);

        contract.reset();

        println!("Valor despues del reset: {}", contract.valor);

        assert_eq!(0, contract.get_num());
    }

}