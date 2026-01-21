// src/monitoring/ghost_vajra_integration.rs

use crate::ghost::ghost_monitor::{GhostBuster};
use crate::entropy::{VajraEntropyMonitor};
use std::sync::Arc;
use serde::{Serialize};

/// Integra detecção de fantasmas com métricas de coerência quântica
pub struct GhostVajraIntegration {
    pub ghost_monitor: Arc<dyn GhostBuster + Send + Sync>,
    pub vajra_monitor: Arc<VajraEntropyMonitor>,

    /// Mapeamento: densidade fantasma → penalidade no phi_score
    pub phantom_to_phi_penalty: f64,
}

impl GhostVajraIntegration {
    pub fn new(
        ghost_monitor: Arc<dyn GhostBuster + Send + Sync>,
        vajra_monitor: Arc<VajraEntropyMonitor>,
    ) -> Self {
        GhostVajraIntegration {
            ghost_monitor,
            vajra_monitor,
            phantom_to_phi_penalty: 0.01, // 1% de redução por fantasma
        }
    }

    /// Processa evento de detecção de fantasma
    pub async fn handle_phantom_detection(
        &mut self,
        phantom_event: PhantomDetectionEvent,
    ) -> IntegrationResponse {
        // 1. Analisar padrão do ataque
        let pattern = self.analyze_attack_pattern(&phantom_event);

        // 2. Calcular penalidade baseada na gravidade
        let penalty = self.calculate_phi_penalty(&phantom_event, &pattern);

        // 3. Aplicar penalidade ao phi_score local
        let new_phi = self.vajra_monitor.adjust_local_phi(
            -penalty,
            PhantomPenaltyReason {
                phantom_density: phantom_event.density,
                attack_pattern: pattern.clone(),
                timestamp: 12345, // Mock timestamp
            }
        ).await;

        // 4. Verificar se ativa contingências
        let contingency_activated = if new_phi < 0.68 {
            self.activate_contingency_measures(&pattern).await;
            true
        } else {
            false
        };

        IntegrationResponse {
            new_phi_score: new_phi,
            penalty_applied: penalty,
            contingency_activated,
            pattern_identified: pattern,
        }
    }

    fn analyze_attack_pattern(
        &self,
        event: &PhantomDetectionEvent,
    ) -> AttackPattern {
        // Classifica o tipo de ataque baseado no padrão do fantasma
        match event.density {
            d if d > 0.9 => AttackPattern::PureGhostInjection,
            d if d > 0.5 => AttackPattern::MixedInjection,
            d if d > 0.2 => AttackPattern::ProbingAttack,
            _ => AttackPattern::Noise,
        }
    }

    fn calculate_phi_penalty(
        &self,
        event: &PhantomDetectionEvent,
        pattern: &AttackPattern,
    ) -> f64 {
        let base_penalty = self.phantom_to_phi_penalty;

        match pattern {
            AttackPattern::PureGhostInjection => base_penalty * 5.0,
            AttackPattern::MixedInjection => base_penalty * 2.0,
            AttackPattern::ProbingAttack => base_penalty * 0.5,
            AttackPattern::Noise => base_penalty * 0.1,
        }
    }

    async fn activate_contingency_measures(
        &mut self,
        pattern: &AttackPattern,
    ) {
        match pattern {
            AttackPattern::PureGhostInjection => {
                // Injeção massiva: selar gateway imediatamente
                self.vajra_monitor.trigger_hard_seal().await;
            }
            AttackPattern::MixedInjection => {
                // Injeção mista: aumentar verificação quântica
                self.vajra_monitor.increase_quantum_validation().await;
            }
            _ => {
                // Ataque leve: apenas registrar
                log::warn!("Ghost attack detected: {:?}", pattern);
            }
        }
    }
}

pub struct PhantomDetectionEvent {
    pub density: f64,
}

pub struct IntegrationResponse {
    pub new_phi_score: f64,
    pub penalty_applied: f64,
    pub contingency_activated: bool,
    pub pattern_identified: AttackPattern,
}

pub struct PhantomPenaltyReason {
    pub phantom_density: f64,
    pub attack_pattern: AttackPattern,
    pub timestamp: u64,
}

/// Tipos de padrões de ataque
#[derive(Debug, Clone, Serialize)]
pub enum AttackPattern {
    PureGhostInjection,  // Fantasmas puros (densidade > 90%)
    MixedInjection,      // Mistura de dados reais e fantasmas
    ProbingAttack,       // Sondagem de vulnerabilidades
    Noise,               // Ruído de fundo
}
