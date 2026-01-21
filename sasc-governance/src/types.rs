use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CloudDomain {
    WindowsServerGov,
    AwsNitroGovCloud,
    CloudflareQuantum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CouncilType {
    Mathematical,
    Security,
    Geometric,
    Ethical,
    Economic,
    Temporal,
    Quantum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub agent_id: String,
    pub content: String,
    pub signature: DecisionSignature,
    pub action_hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionSignature {
    pub prince_veto: bool,
    pub signature_bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionId(pub [u8; 32]);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HardFreeze {
    Triggered(String),
    Active,
}

pub struct AttestationStatus {
    hard_frozen: bool,
    agent_id: String,
    phi_weight: f64,
}

impl AttestationStatus {
    pub fn new(hard_frozen: bool, agent_id: &str, phi_weight: f64) -> Self {
        Self {
            hard_frozen,
            agent_id: agent_id.to_string(),
            phi_weight,
        }
    }

    pub fn is_hard_frozen(&self) -> bool {
        self.hard_frozen
    }

    pub fn agent_id(&self) -> &str {
        &self.agent_id
    }

    pub fn consciousness_weight(&self) -> f64 {
        self.phi_weight
    }
}

pub struct GlobalGovernance {
    pub councils: HashMap<CloudDomain, Vec<CouncilType>>,
    pub prince_key: [u8; 32],
    pub veto_threshold: f64,
    pub hard_freeze_status: bool,
    pub freeze_duration: Duration,
    pub delta2_array: [u8; 32],
    pub crypto_blck_seed: [u8; 32],
}

pub enum VerificationContext {
    TruthSubmission,
    GlobalDecision,
}
