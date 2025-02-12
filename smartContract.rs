#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait VotingContract {
    #[endpoint]
    fn create_voting(&self, voting_id: u64, question: ManagedBuffer) {
        require!(!self.votings().contains_key(&voting_id), "La votació ja existeix!");
        self.votings().insert(voting_id, &question);
        self.yes_votes(voting_id).set(0);
        self.no_votes(voting_id).set(0);
    }

    #[endpoint]
    fn vote(&self, voting_id: u64, nif_hash: ManagedBuffer, choice: ManagedBuffer) {
        require!(self.votings().contains_key(&voting_id), "Votació inexistent!");
        require!(!self.voted_hashes().contains(&nif_hash), "Ja has votat en aquesta votació!");

        // Guardar el hash del NIF per evitar que voti més d'una vegada
        self.voted_hashes().insert(nif_hash);

        // Comptabilitzar el vot
        if choice == ManagedBuffer::from(b"Sí") {
            self.yes_votes(voting_id).update(|v| *v += 1);
        } else if choice == ManagedBuffer::from(b"No") {
            self.no_votes(voting_id).update(|v| *v += 1);
        }
    }

    #[view]
    fn get_results(&self, voting_id: u64) -> (u64, u64) {
        require!(self.votings().contains_key(&voting_id), "Votació inexistent!");
        (self.yes_votes(voting_id).get(), self.no_votes(voting_id).get())
    }

    #[view]
    fn get_question(&self, voting_id: u64) -> ManagedBuffer {
        require!(self.votings().contains_key(&voting_id), "Votació inexistent!");
        self.votings().get(&voting_id).unwrap()
    }

    #[storage_mapper("votings")]
    fn votings(&self) -> MapMapper<u64, ManagedBuffer>;

    #[storage_mapper("yes_votes")]
    fn yes_votes(&self, voting_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("no_votes")]
    fn no_votes(&self, voting_id: u64) -> SingleValueMapper<u64>;

    #[storage_mapper("voted_hashes")]
    fn voted_hashes(&self) -> UnorderedSetMapper<ManagedBuffer>;
}
