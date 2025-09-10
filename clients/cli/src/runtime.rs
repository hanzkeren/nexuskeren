//! Simplified runtime for coordinating authenticated workers

use crate::environment::Environment;
use crate::events::Event;
use crate::orchestrator::OrchestratorClient;

use crate::workers::core::WorkerConfig;
use crate::workers::parallel_runner::ParallelRunner;
use ed25519_dalek::SigningKey;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;

/// Start parallel authenticated workers utilizing all CPU cores
#[allow(clippy::too_many_arguments)]
pub async fn start_parallel_authenticated_workers(
    node_id: u64,
    signing_key: SigningKey,
    orchestrator: OrchestratorClient,
    shutdown: broadcast::Receiver<()>,
    environment: Environment,
    client_id: String,
    max_threads: Option<u32>,
    max_tasks: Option<u32>,
    max_difficulty: Option<crate::nexus_orchestrator::TaskDifficulty>,
) -> (
    mpsc::Receiver<Event>,
    Vec<JoinHandle<()>>,
    broadcast::Sender<()>,
) {
    let mut config = WorkerConfig::new(environment, client_id);
    config.max_difficulty = max_difficulty;

    // Create the parallel runner
    let parallel_runner = ParallelRunner::new(max_threads, max_tasks);
    
    // Print system information
    println!("{}", ParallelRunner::get_system_info());
    
    // Start parallel workers
    parallel_runner
        .start_parallel_workers(
            node_id,
            signing_key,
            orchestrator,
            config,
            shutdown,
            max_difficulty,
        )
        .await
}

/// Start single authenticated worker (legacy function for compatibility)
#[allow(clippy::too_many_arguments)]
pub async fn start_authenticated_worker(
    node_id: u64,
    signing_key: SigningKey,
    orchestrator: OrchestratorClient,
    shutdown: broadcast::Receiver<()>,
    environment: Environment,
    client_id: String,
    max_tasks: Option<u32>,
    max_difficulty: Option<crate::nexus_orchestrator::TaskDifficulty>,
) -> (
    mpsc::Receiver<Event>,
    Vec<JoinHandle<()>>,
    broadcast::Sender<()>,
) {
    // Use parallel workers with single thread for backward compatibility
    start_parallel_authenticated_workers(
        node_id,
        signing_key,
        orchestrator,
        shutdown,
        environment,
        client_id,
        Some(1), // Force single worker for compatibility
        max_tasks,
        max_difficulty,
    )
    .await
}
