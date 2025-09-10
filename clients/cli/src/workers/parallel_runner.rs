//! Parallel task runner that utilizes all available CPU cores
//! 
//! This module provides functionality to run multiple authenticated workers
//! in parallel, maximizing CPU utilization for proof generation tasks.

use super::authenticated_worker::AuthenticatedWorker;
use super::core::WorkerConfig;
use crate::events::Event;
use crate::orchestrator::OrchestratorClient;
use ed25519_dalek::SigningKey;
use sysinfo::System;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;

/// Parallel runner that manages multiple worker threads
pub struct ParallelRunner {
    /// Number of worker threads to spawn
    pub num_workers: usize,
    /// Maximum number of tasks per worker (if specified)
    pub max_tasks_per_worker: Option<u32>,
}

impl ParallelRunner {
    /// Create a new parallel runner with optimal worker count
    pub fn new(max_threads: Option<u32>, max_tasks: Option<u32>) -> Self {
        let num_cores = Self::get_optimal_worker_count();
        let num_workers = if let Some(max) = max_threads {
            // Respect user's max_threads but don't exceed available cores
            (max as usize).min(num_cores)
        } else {
            // Use all available cores by default
            num_cores
        };

        // Distribute total tasks across workers if max_tasks is specified
        let max_tasks_per_worker = max_tasks.map(|total| {
            if num_workers == 0 {
                total
            } else {
                // Distribute tasks evenly, with remainder going to first workers
                (total + num_workers as u32 - 1) / num_workers as u32
            }
        });

        Self {
            num_workers,
            max_tasks_per_worker,
        }
    }

    /// Get optimal worker count based on CPU cores and system resources
    fn get_optimal_worker_count() -> usize {
        let mut sys = System::new_all();
        sys.refresh_cpu_all();
        
        let logical_cores = sys.cpus().len();
        
        // For CPU-intensive tasks like proof generation, we typically want
        // one worker per logical core for optimal performance
        // However, we'll apply some constraints based on available memory
        let memory_gb = sys.total_memory() / (1024 * 1024 * 1024); // Convert to GB
        
        // Estimate memory usage per worker (conservative estimate)
        let estimated_memory_per_worker_gb = 2; // 2GB per worker as conservative estimate
        let max_workers_by_memory = if memory_gb > 0 {
            (memory_gb / estimated_memory_per_worker_gb as u64).max(1) as usize
        } else {
            logical_cores
        };

        // Use the minimum of cores and memory-constrained workers
        // but ensure at least 1 worker
        logical_cores.min(max_workers_by_memory).max(1)
    }

    /// Start multiple authenticated workers in parallel
    #[allow(clippy::too_many_arguments)]
    pub async fn start_parallel_workers(
        &self,
        node_id: u64,
        _signing_key: SigningKey,
        orchestrator: OrchestratorClient,
        config: WorkerConfig,
        shutdown: broadcast::Receiver<()>,
        _max_difficulty: Option<crate::nexus_orchestrator::TaskDifficulty>,
    ) -> (
        mpsc::Receiver<Event>,
        Vec<JoinHandle<()>>,
        broadcast::Sender<()>,
    ) {
        let (combined_event_sender, combined_event_receiver) = 
            mpsc::channel::<Event>(crate::consts::cli_consts::EVENT_QUEUE_SIZE * self.num_workers);
        
        // Create shutdown sender for max tasks completion
        let (max_tasks_shutdown_sender, _) = broadcast::channel(1);
        
        let mut all_join_handles = Vec::new();
        
        println!("Starting {} parallel workers across {} CPU cores", 
                 self.num_workers, Self::get_optimal_worker_count());

        for worker_id in 0..self.num_workers {
            // Create individual event channel for this worker
            let (worker_event_sender, mut worker_event_receiver) = 
                mpsc::channel::<Event>(crate::consts::cli_consts::EVENT_QUEUE_SIZE);
            
            // Clone necessary components for this worker
            let worker_config = config.clone();
            let worker_orchestrator = orchestrator.clone();
            let worker_shutdown = shutdown.resubscribe();
            let mut worker_max_tasks_shutdown = max_tasks_shutdown_sender.subscribe();
            
            // Create signing key for this worker (each worker needs its own key)
            let mut csprng = rand_core::OsRng;
            let worker_signing_key = SigningKey::generate(&mut csprng);
            
            // Create the worker
            let worker = AuthenticatedWorker::new(
                node_id + worker_id as u64, // Unique node ID per worker
                worker_signing_key,
                worker_orchestrator,
                worker_config,
                worker_event_sender,
                self.max_tasks_per_worker,
                max_tasks_shutdown_sender.clone(),
            );
            
            // Start the worker
            let worker_handles = worker.run(worker_shutdown).await;
            all_join_handles.extend(worker_handles);
            
            // Forward events from worker to combined channel
            let combined_sender = combined_event_sender.clone();
            let event_forwarder = tokio::spawn(async move {
                loop {
                    tokio::select! {
                        _ = worker_max_tasks_shutdown.recv() => break,
                        event = worker_event_receiver.recv() => {
                            match event {
                                                                 Some(mut event) => {
                                     // Tag events with worker ID for debugging
                                     event.msg = format!("[W{}] {}", worker_id, event.msg);
                                     if combined_sender.send(event).await.is_err() {
                                         break; // Main receiver closed
                                     }
                                 }
                                None => break, // Worker event sender closed
                            }
                        }
                    }
                }
            });
            all_join_handles.push(event_forwarder);
        }

        (combined_event_receiver, all_join_handles, max_tasks_shutdown_sender)
    }

    /// Get system information for debugging
    pub fn get_system_info() -> String {
        let mut sys = System::new_all();
        sys.refresh_cpu_all();
        sys.refresh_memory();
        
        let logical_cores = sys.cpus().len();
        let total_memory_gb = sys.total_memory() / (1024 * 1024 * 1024);
        let available_memory_gb = sys.available_memory() / (1024 * 1024 * 1024);
        
        format!(
            "System Info: {} logical cores, {} GB total memory, {} GB available memory",
            logical_cores, total_memory_gb, available_memory_gb
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimal_worker_count() {
        let count = ParallelRunner::get_optimal_worker_count();
        assert!(count >= 1, "Should have at least 1 worker");
        assert!(count <= 128, "Should not exceed reasonable limits"); // Sanity check
    }

    #[test]
    fn test_parallel_runner_creation() {
        // Test with no limits
        let runner = ParallelRunner::new(None, None);
        assert!(runner.num_workers >= 1);
        assert_eq!(runner.max_tasks_per_worker, None);

        // Test with thread limit
        let runner = ParallelRunner::new(Some(4), None);
        assert!(runner.num_workers <= 4);
        assert!(runner.num_workers >= 1);

        // Test with task limit
        let runner = ParallelRunner::new(Some(4), Some(100));
        assert!(runner.num_workers <= 4);
        if let Some(tasks_per_worker) = runner.max_tasks_per_worker {
            assert!(tasks_per_worker > 0);
            assert!(tasks_per_worker * runner.num_workers as u32 >= 100);
        }
    }

    #[test]
    fn test_system_info() {
        let info = ParallelRunner::get_system_info();
        assert!(info.contains("logical cores"));
        assert!(info.contains("total memory"));
        assert!(info.contains("available memory"));
    }
}
