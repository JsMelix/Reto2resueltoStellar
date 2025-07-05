#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, log, Address, Env};
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    // Quien creó la votación
    Creator,
    // Si la votación está activa
    Active,
    // Cuántos votos tiene "SI"
    VotesSi,
    // Cuántos votos tiene "NO"
    VotesNo,
    // Si una persona ya votó
    HasVoted(Address),
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Vote {
    Si,
    No,
}

#[contracterror]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// El contrato ya ha sido inicializado.
    AlreadyInitialized = 1,
    /// El contrato no ha sido inicializado.
    NotInitialized = 2,
    /// El período de votación no está activo.
    VotingNotActive = 3,
    /// La dirección ya ha votado.
    AlreadyVoted = 4,
    /// Quien llama no es el creador de la votación.
    NotCreator = 5,
}

#[contract]
pub struct SimpleVoting;

#[contractimpl]
impl SimpleVoting {
    /// Inicializar la votación (solo una vez)
    pub fn init(env: Env, creator: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Creator) {
            return Err(Error::AlreadyInitialized);
        }

        // El creador debe autorizar
        creator.require_auth();

        log!(&env, "Iniciando votación UUUUUUUUUUU, creador: {}", creator);

        // Guardar datos iniciales
        env.storage().instance().set(&DataKey::Creator, &creator);
        env.storage().instance().set(&DataKey::Active, &true);
        env.storage().instance().set(&DataKey::VotesSi, &0u32);
        env.storage().instance().set(&DataKey::VotesNo, &0u32);

        log!(&env, "Votación inicializada correctamente");
        Ok(())
    }

    /// Votar SI
    pub fn vote_si(env: Env, voter: Address) -> Result<(), Error> {
        Self::_vote(env, voter, Vote::Si)
    }

    /// Votar NO
    pub fn vote_no(env: Env, voter: Address) -> Result<(), Error> {
        Self::_vote(env, voter, Vote::No)
    }

    /// Cerrar votación (solo el creador)
    pub fn close_voting(env: Env, creator: Address) -> Result<(), Error> {
        creator.require_auth();

        log!(&env, "Cerrando votación...");

        // Verificar que sea el creador
        let stored_creator: Address = env
            .storage()
            .instance()
            .get(&DataKey::Creator)
            .ok_or(Error::NotInitialized)?;

        if stored_creator != creator {
            return Err(Error::NotCreator);
        }

        // Cerrar votación
        env.storage().instance().set(&DataKey::Active, &false);

        log!(&env, "Votación cerrada");
        Ok(())
    }

    // --- Funciones privadas de ayuda ---

    fn _vote(env: Env, voter: Address, vote: Vote) -> Result<(), Error> {
        // El votante debe autorizar
        voter.require_auth();

        log!(&env, "Usuario {} votando {:?}", voter, vote);

        // Verificar que la votación esté activa
        let active: bool = env
            .storage()
            .instance()
            .get(&DataKey::Active)
            .ok_or(Error::NotInitialized)?;

        if !active {
            return Err(Error::VotingNotActive);
        }

        // Verificar que no haya votado antes
        let has_voted_key = DataKey::HasVoted(voter.clone());
        if env.storage().instance().has(&has_voted_key) {
            return Err(Error::AlreadyVoted);
        }

        // Registrar que votó
        env.storage().instance().set(&has_voted_key, &true);

        // Incrementar el contador de votos y registrar el evento
        match vote {
            Vote::Si => {
                let key = DataKey::VotesSi;
                let current_votes: u32 = env.storage().instance().get(&key).unwrap_or(0);
                let new_votes = current_votes + 1;
                env.storage().instance().set(&key, &new_votes);
                log!(&env, "Voto SI registrado. Total votos SI: {}", new_votes);
            }
            Vote::No => {
                let key = DataKey::VotesNo;
                let current_votes: u32 = env.storage().instance().get(&key).unwrap_or(0);
                let new_votes = current_votes + 1;
                env.storage().instance().set(&key, &new_votes);
                log!(&env, "Voto NO registrado. Total votos NO: {}", new_votes);
            }
        };
        Ok(())
    }

    // --- Funciones de solo lectura ---

    /// Ver resultados
    pub fn get_results(env: Env) -> (u32, u32, bool) {
        let votes_si: u32 = env.storage().instance().get(&DataKey::VotesSi).unwrap_or(0);

        let votes_no: u32 = env.storage().instance().get(&DataKey::VotesNo).unwrap_or(0);

        let active: bool = env
            .storage()
            .instance()
            .get(&DataKey::Active)
            .unwrap_or(false);

        (votes_si, votes_no, active)
    }

    /// Verificar si alguien ya votó
    pub fn has_voted(env: Env, user: Address) -> bool {
        env.storage().instance().has(&DataKey::HasVoted(user))
    }
}

mod test;
