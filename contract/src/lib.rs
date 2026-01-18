#[derive(Debug, PartialEq)]
pub enum EscrowError {
    Unauthorized,
    InvalidState(String),
    InsufficientFunds,
    AlreadyInitialized,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EscrowState {
    AwaitingPayment,
    AwaitingDelivery,
    Completed,
    Refunded,
}

pub struct Escrow {
    pub buyer: String,
    pub seller: String,
    pub arbiter: Option<String>,
    pub amount: u64,
    pub balance: u64,
    pub state: EscrowState,
}

impl Escrow {
    pub fn new(buyer: String, seller: String, amount: u64) -> Self {
        Escrow {
            buyer,
            seller,
            arbiter: None,
            amount,
            balance: 0,
            state: EscrowState::AwaitingPayment,
        }
    }

    pub fn deposit(&mut self, sender: &str, amount: u64) -> Result<(), EscrowError> {
        if self.state != EscrowState::AwaitingPayment {
            return Err(EscrowError::InvalidState("Deposit only allowed in AwaitingPayment state".into()));
        }
        
        if sender != self.buyer {
            return Err(EscrowError::Unauthorized); 
        }

        if amount != self.amount {
             return Err(EscrowError::InsufficientFunds);
        }

        self.balance += amount;
        self.state = EscrowState::AwaitingDelivery;
        
        Ok(())
    }

    pub fn confirm_delivery(&mut self, sender: &str) -> Result<u64, EscrowError> {
        if self.state != EscrowState::AwaitingDelivery {
            return Err(EscrowError::InvalidState("Cannot confirm delivery unless funds are deposited".into()));
        }

        if sender != self.buyer {
            return Err(EscrowError::Unauthorized);
        }

        let payment = self.balance;
        self.balance = 0;
        self.state = EscrowState::Completed;

        Ok(payment)
    }

    pub fn refund_buyer(&mut self, sender: &str) -> Result<u64, EscrowError> {
        if self.state != EscrowState::AwaitingDelivery {
             return Err(EscrowError::InvalidState("Can only refund when funds are held".into()));
        }

        if sender != self.seller {
            return Err(EscrowError::Unauthorized);
        }

        let refund_amount = self.balance;
        self.balance = 0;
        self.state = EscrowState::Refunded;
        
        Ok(refund_amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let contract = Escrow::new("alice".to_string(), "bob".to_string(), 100);
        assert_eq!(contract.state, EscrowState::AwaitingPayment);
        assert_eq!(contract.balance, 0);
    }

    #[test]
    fn test_deposit_flow() {
        let mut contract = Escrow::new("alice".to_string(), "bob".to_string(), 100);
        
        assert_eq!(contract.deposit("bob", 100), Err(EscrowError::Unauthorized));

        assert!(contract.deposit("alice", 100).is_ok());
        assert_eq!(contract.state, EscrowState::AwaitingDelivery);
        assert_eq!(contract.balance, 100);
    }

    #[test]
    fn test_release_flow() {
        let mut contract = Escrow::new("alice".to_string(), "bob".to_string(), 100);
        contract.deposit("alice", 100).unwrap();

        assert_eq!(contract.confirm_delivery("bob"), Err(EscrowError::Unauthorized));

        let result = contract.confirm_delivery("alice");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);
        assert_eq!(contract.state, EscrowState::Completed);
        assert_eq!(contract.balance, 0);
    }

    #[test]
    fn test_refund_flow() {
        let mut contract = Escrow::new("alice".to_string(), "bob".to_string(), 100);
        contract.deposit("alice", 100).unwrap();

        assert_eq!(contract.refund_buyer("alice"), Err(EscrowError::Unauthorized));

        let result = contract.refund_buyer("bob");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);
        assert_eq!(contract.state, EscrowState::Refunded);
    }
}
