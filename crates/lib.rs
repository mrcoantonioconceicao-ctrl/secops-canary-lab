pub struct Wallet {
    pub balance: u64,
}

impl Wallet {
    pub fn withdraw(&mut self, amount: u64) -> Result<(), &'static str> {
        // Vulnerabilidade de subtração sem verificação
                if amount > self.balance {
            return Err("Insufficient funds");
        }
        self.balance = self.balance - amount;
        Ok(())
    }
}
