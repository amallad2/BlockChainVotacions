#![no_std]

use multiversx_sc::derive_imports::*;
#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed,
}

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait CrowdfundingSc {
    #[init]
    fn init(&self, target: BigUint, deadline: u64, min_amount: BigUint, max_amount_per_donor: BigUint, max_amount_total: BigUint) {
        require!(target > 0, "Target must be more than 0");
        self.target().set(target);

        require!(
            deadline > self.get_current_time(),
            "Deadline can't be in the past"
        );
        self.deadline().set(deadline);

        require!(min_amount > 0, "Minimum amount must be more than 0");
        self.min_amount().set(min_amount);

        require!(max_amount_per_donor > 0, "Maximum amount per donor must be more than 0");
        self.max_amount_per_donor().set(max_amount_per_donor);

        require!(max_amount_total > 0, "Maximum amount total must be more than 0");
        self.max_amount_total().set(max_amount_total);
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint]
    #[payable("EGLD")]
    fn fund(&self) {
        let payment = self.call_value().egld_value().clone_value();

        require!(
            payment >= self.min_amount().get(),
            "The amount is less than the minimum allowed"
        );

        let current_time = self.blockchain().get_block_timestamp();
        require!(
            current_time < self.deadline().get(),
            "cannot fund after deadline"
        );

        let caller = self.blockchain().get_caller();
        let deposited_amount = self.deposit(&caller).get();
        let new_deposit = deposited_amount + payment.clone();

        require!(
            new_deposit <= self.max_amount_per_donor().get(),
            "The amount exceeds the maximum allowed per donor"
        );

        let total_funds = self.get_current_funds() + payment;
        require!(
            total_funds <= self.max_amount_total().get(),
            "The amount exceeds the maximum allowed for the project"
        );

        self.deposit(&caller).set(new_deposit);
    }

    #[endpoint]
    fn claim(&self) {
        match self.status() {
            Status::FundingPeriod => sc_panic!("cannot claim before deadline"),
            Status::Successful => {
                let caller = self.blockchain().get_caller();
                require!(
                    caller == self.blockchain().get_owner_address(),
                    "only owner can claim successful funding"
                );

                let sc_balance = self.get_current_funds();
                self.send().direct_egld(&caller, &sc_balance);
            }
            Status::Failed => {
                let caller = self.blockchain().get_caller();
                let deposit = self.deposit(&caller).get();

                if deposit > 0u32 {
                    self.deposit(&caller).clear();
                    self.send().direct_egld(&caller, &deposit);
                }
            }
        }
    }

    #[view]
    fn status(&self) -> Status {
        if self.get_current_time() <= self.deadline().get() {
            Status::FundingPeriod
        } else if self.get_current_funds() >= self.target().get() {
            Status::Successful
        } else {
            Status::Failed
        }
    }

    #[view(getCurrentFunds)]
    fn get_current_funds(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0)
    }

    // private

    fn get_current_time(&self) -> u64 {
        self.blockchain().get_block_timestamp()
    }

    // storage

    #[view(getTarget)]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;

    #[view(getDeadline)]
    #[storage_mapper("deadline")]
    fn deadline(&self) -> SingleValueMapper<u64>;

    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getMinAmount)]
    #[storage_mapper("min_amount")]
    fn min_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getMaxAmountPerDonor)]
    #[storage_mapper("max_amount_per_donor")]
    fn max_amount_per_donor(&self) -> SingleValueMapper<BigUint>;

    #[view(getMaxAmountTotal)]
    #[storage_mapper("max_amount_total")]
    fn max_amount_total(&self) -> SingleValueMapper<BigUint>;
}