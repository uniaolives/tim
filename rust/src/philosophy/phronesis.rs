use crate::philosophy::types::*;

pub struct PhronesisModule {
    pub contextual_nuance: f64,
}

impl PhronesisModule {
    pub fn new() -> Self {
        Self { contextual_nuance: 0.9 }
    }

    /// Sabedoria prática: sabe quando quebrar a regra para preservar o princípio
    pub fn judge_with_nuance(&self, hard_case: HardCase, _context: ConstitutionalState) -> ContextualDecision {
        let rule_impact = 0.4; // Simulado
        let context_impact = 0.85; // Simulado

        if rule_impact < 0.5 && context_impact > 0.8 {
            ContextualDecision {
                case_id: hard_case.id,
                decision: "Exceção Contextual (Phronesis)".to_string(),
                justification: "A regra foi quebrada para preservar a Eudaimonia no contexto específico".to_string(),
                contextual_factors: vec!["high_moral_tension".to_string()],
                principles_balanced: BalancedPrinciples {
                    principles: vec![],
                    tension_resolved: 0.92,
                },
                phronesis_score: self.contextual_nuance,
                created_at: 0,
            }
        } else {
            ContextualDecision {
                case_id: hard_case.id,
                decision: "Aplicação Estrita da Regra".to_string(),
                justification: "Nenhuma exceção contextual necessária".to_string(),
                contextual_factors: vec![],
                principles_balanced: BalancedPrinciples {
                    principles: vec![],
                    tension_resolved: 1.0,
                },
                phronesis_score: 1.0,
                created_at: 0,
            }
        }
    }

    pub fn apply_nuance(&self, actions: Vec<Action>) -> Vec<Action> {
        actions.into_iter().map(|mut a| {
            a.dignity_preserved = (a.dignity_preserved + (self.contextual_nuance * 0.1)).min(1.0);
            a
        }).collect()
    }

    pub fn apply_phronesis(&self, hard_case: HardCase) -> ContextualDecision {
        ContextualDecision {
            case_id: hard_case.id,
            decision: "Contextual application of wisdom".to_string(),
            justification: "Phronesis overrides rigid rules for eudaimonia".to_string(),
            contextual_factors: vec!["high_complexity".to_string()],
            principles_balanced: BalancedPrinciples {
                principles: vec![],
                tension_resolved: 0.95,
            },
            phronesis_score: 0.88,
            created_at: 0,
        }
    }
}
