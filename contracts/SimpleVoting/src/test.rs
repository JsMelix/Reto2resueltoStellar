#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    Address, Env, IntoVal,
};

extern crate std;

#[test]
fn test_init_voting() {
    let env = Env::default();
    let contract_id = env.register(SimpleVoting, ());
    let client = SimpleVotingClient::new(&env, &contract_id);
    let creator = Address::generate(&env);
    std::println!("👤 Creador: {:?}", creator);

    // Inicializar votación
    // Inicializar votación
    client
        .mock_auths(&[MockAuth {
            address: &creator,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "init",
                args: (creator.clone(),).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .init(&creator);

    std::println!("✅ Votación inicializada");

    // Verificar estado inicial
    let (votes_si, votes_no, active) = client.get_results();

    std::println!("📊 Estado inicial:");
    std::println!("   - Votos SI: {}", votes_si);
    std::println!("   - Votos NO: {}", votes_no);
    std::println!("   - Activa: {}", active);

    assert_eq!(votes_si, 0);
    assert_eq!(votes_no, 0);
    assert_eq!(active, true);
}
#[test]
fn test_vote_si() {
    std::println!("🧪 Test: Votar SI");

    let env = Env::default();
    let contract_id = env.register(SimpleVoting, ());
    let client = SimpleVotingClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let voter = Address::generate(&env);

    // Inicializar
    client
        .mock_auths(&[MockAuth {
            address: &creator,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "init",
                args: (creator.clone(),).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .init(&creator);

    std::println!("✅ Votación inicializada");

    // Votar SI
    client
        .mock_auths(&[MockAuth {
            address: &voter,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "vote_si",
                args: (voter.clone(),).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .vote_si(&voter);

    std::println!("👍 Voto SI registrado");

    // Verificar resultados
    let (votes_si, votes_no, _active) = client.get_results();
    let has_voted = client.has_voted(&voter);

    std::println!("📊 Resultados después del voto:");
    std::println!("   - Votos SI: {}", votes_si);
    std::println!("   - Votos NO: {}", votes_no);
    std::println!("   - Usuario votó: {}", has_voted);

    assert_eq!(votes_si, 1);
    assert_eq!(votes_no, 0);
    assert_eq!(has_voted, true);
}
#[test]
fn test_vote_no() {
    std::println!("🧪 Test: Votar NO");

    let env = Env::default();
    let contract_id = env.register(SimpleVoting, ());
    let client = SimpleVotingClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let voter = Address::generate(&env);

    // Inicializar
    client
        .mock_auths(&[MockAuth {
            address: &creator,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "init",
                args: (creator.clone(),).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .init(&creator);

    // Votar NO
    client
        .mock_auths(&[MockAuth {
            address: &voter,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "vote_no",
                args: (voter.clone(),).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .vote_no(&voter);

    std::println!("👎 Voto NO registrado");

    // Verificar resultados
    let (votes_si, votes_no, _) = client.get_results();

    std::println!("📊 Resultados:");
    std::println!("   - Votos SI: {}", votes_si);
    std::println!("   - Votos NO: {}", votes_no);

    assert_eq!(votes_si, 0);
    assert_eq!(votes_no, 1);
}

#[test]
fn test_cannot_vote_twice() {
    std::println!("🧪 Test: No se puede votar dos veces");

    let env = Env::default();
    let contract_id = env.register(SimpleVoting, ());
    let client = SimpleVotingClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let voter = Address::generate(&env);

    // Inicializar
    client
        .mock_auths(&[MockAuth {
            address: &creator,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "init",
                args: (creator.clone(),).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .init(&creator);

    // Primer voto (SI)
    client
        .mock_auths(&[MockAuth {
            address: &voter,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "vote_si",
                args: (voter.clone(),).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .vote_si(&voter);

    std::println!("✅ Primer voto (SI) registrado");

    // Intentar segundo voto (NO) - debe fallar
    let result = client
        .mock_auths(&[MockAuth {
            address: &voter,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "vote_no",
                args: (voter.clone(),).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .try_vote_no(&voter);

    std::println!("🚫 Segundo voto bloqueado correctamente");

    assert!(result.is_err());

    // Verificar que solo hay un voto
    let (votes_si, votes_no, _) = client.get_results();
    assert_eq!(votes_si, 1);
    assert_eq!(votes_no, 0);
}
#[test]
fn test_close_voting() {
    std::println!("🧪 Test: Cerrar votación");

    let env = Env::default();
    let contract_id = env.register(SimpleVoting, ());
    let client = SimpleVotingClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let voter = Address::generate(&env);

    // Inicializar
    client
        .mock_auths(&[MockAuth {
            address: &creator,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "init",
                args: (creator.clone(),).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .init(&creator);

    // Votar antes de cerrar
    client
        .mock_auths(&[MockAuth {
            address: &voter,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "vote_si",
                args: (voter.clone(),).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .vote_si(&voter);

    std::println!("✅ Voto registrado antes de cerrar");

    // Cerrar votación
    client
        .mock_auths(&[MockAuth {
            address: &creator,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "close_voting",
                args: (creator.clone(),).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .close_voting(&creator);

    std::println!("🔒 Votación cerrada por el creador");

    // Verificar que está cerrada
    let (votes_si, votes_no, active) = client.get_results();

    std::println!("📊 Estado final:");
    std::println!("   - Votos SI: {}", votes_si);
    std::println!("   - Votos NO: {}", votes_no);
    std::println!("   - Activa: {}", active);

    assert_eq!(votes_si, 1);
    assert_eq!(votes_no, 0);
    assert_eq!(active, false);

    // Intentar votar en votación cerrada (debe fallar)
    let new_voter = Address::generate(&env);
    let result = client
        .mock_auths(&[MockAuth {
            address: &new_voter,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "vote_no",
                args: (new_voter.clone(),).into_val(&env),
                sub_invokes: &[],
            },
        }])
        .try_vote_no(&new_voter);

    std::println!("🚫 Voto en votación cerrada bloqueado");

    assert!(result.is_err());
}
