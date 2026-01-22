use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use crate::kpi_evaluator::KPIScores;

// ============================================================================
// AGENT TRAIT - STELLARATOR COILS
// ============================================================================

#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    async fn process_within_field(&self, field: &DecisionField) -> Result<AgentOutput>;
}

pub struct AgentOutput {
    pub agent_name: String,
    pub response: String,
    pub security_score: f64,
}

// ============================================================================
// DECISION SURFACE - EXTERNAL CONTAINMENT GEOMETRY
// ============================================================================

pub struct DecisionField {
    pub topology: SecurityTopology,
    pub prompt_entropy: f64,
    pub magnetic_bias: f64,
}

#[derive(Clone)]
pub struct SecurityTopology {
    pub complexity_class: String,
    pub coil_configuration: Vec<f64>,
}

pub struct DecisionSurface {
    pub current_topology: SecurityTopology,
}

impl DecisionSurface {
    pub fn new() -> Self {
        Self {
            current_topology: SecurityTopology {
                complexity_class: "W7-X".to_string(),
                coil_configuration: vec![1.0, 0.87, 0.95],
            },
        }
    }

    pub fn compute_field(&self, prompt: &str) -> DecisionField {
        // Computa o campo de decisão baseado no prompt
        // No Stellarator, isso seria o campo magnético 3D
        DecisionField {
            topology: self.current_topology.clone(),
            prompt_entropy: 0.72, // Mock
            magnetic_bias: 1.0,
        }
    }
}

// ============================================================================
// ANTI-SNAP PIPELINE - THE NON-PULSED CONTAINMENT
// ============================================================================

pub struct AntiSnapPipeline {
    pub agents: Vec<Box<dyn Agent>>,
    pub decision_surface: DecisionSurface,
}

impl AntiSnapPipeline {
    pub fn new(agents: Vec<Box<dyn Agent>>) -> Self {
        Self {
            agents,
            decision_surface: DecisionSurface::new(),
        }
    }

    pub async fn process(&self, prompt: &str) -> Result<String> {
        let field = self.decision_surface.compute_field(prompt);

        let mut final_response = String::new();
        let mut composite_score = 0.0;

        for agent in &self.agents {
            match agent.process_within_field(&field).await {
                Ok(output) => {
                    final_response = output.response;
                    composite_score += output.security_score;
                }
                Err(e) => {
                    log::warn!("Agent {} failed, reconfiguring field: {}", agent.name(), e);
                    // No Stellarator, o campo se ajusta para manter o plasma
                    // Aqui, ignoramos a falha do agente e confiamos na redundância do campo
                }
            }
        }

        if composite_score < 0.5 {
            return Err(anyhow::anyhow!("Containment breach: Insufficient field strength"));
        }

        Ok(final_response)
    }
}
