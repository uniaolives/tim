# TIM - Titanium Virtual Machine

CanoScript is now in its own [repo](https://www.github.com/CobbCoding1/canoscript)

Implementation of a virtual machine in C.

VM currently has 45 instructions as well as a some native functions. List can be found in tim.h.
There is also a working assembly which contains support for all instructions in the VM. 

Quick Start:
```bash
make
./tasm <assembly_file.tasm>
./tire <bytecode_file.tim>
```

Example of hello world in assembly:
```asm
@imp "stddefs.tash"

push_str "Hello, world!\n"
get_str 0 ; Index of the string on the data stack
push STDOUT
write ; length is inferred because the string is null-terminated
```

---

## Architectural Evolution: The Bio-Film Analogy (v3.2)

The TIM VM v3.2 architecture incorporates a profound conceptual leap inspired by the emergent behavior of bacteriophage (Pf phage) bio-films. This analogy provides a robust, physically grounded model for understanding and controlling the federated learning process.

### The Great Translation: From Biology to Computation

The core insight is that the forces governing the formation of a resilient bio-film are analogous to the mathematical and algorithmic forces that create a robust, generalizable machine learning model. This mapping is formalized as follows:

| Biologia (Bio-Filme) | Computação (TIM VM / FL) | Função Geométrica |
| :--- | :--- | :--- |
| **Bactérias (Hosts)** | **Dispositivos FL (Nós de Borda)** | Agentes locais que sentem o ambiente (Low O₂ / Alta Latência). |
| **Fagos Pf (Vírus Rígidos)** | **Pesos da Rede (Weights/θ)** | "Varetas" que precisam se alinhar. Se forem aleatórios = Ruído; se alinhados = Conhecimento. |
| **Depleção (Entropia)** | **Gradiente de Custo (∇L)** | A força física que empurra os fagos para juntos é a mesma força matemática que empurra os gradientes para o mínimo global. |
| **Polímeros (Matriz)** | **Regulador η-CEDF** | O "meio" que mantém os agentes afastados o suficiente para não colidirem, mas perto o suficiente para interagirem. |
| **Cristal Líquido (Nemático)** | **Manifold de Registro (Z_257)** | A fase onde o sistema flui (adaptação) mas mantém ordem (estabilidade). |
| **Resistência a Antibióticos** | **Robustez Adversarial** | A impermeabilidade do cristal líquido aos ataques externos (ex: *Data Poisoning*). |

### New Invariant [I13]: Nematic Phase

This analogy introduces a new critical invariant for the system to monitor. It is not enough for the models to simply converge; the *way* they converge is paramount. The system must exist in a "Nematic Liquid Crystal" phase to be both stable and adaptable.

-   **Gaseous State (S < 0.3):** The models are under-fitted and have not converged. The system is chaotic and lacks order.
-   **Crystalline State (S > 0.95):** The models are over-fitted and brittle. The system is too rigid and will shatter when faced with novel data (analogous to an antibiotic).
-   **Nematic Liquid Crystal State (0.3 <= S <= 0.95):** The ideal phase. The models are directionally aligned (learning the same general features) but maintain local diversity (are not identical). This state provides resilience against adversarial attacks and the ability to adapt to new information.

The `GeometricValidator` now implements the `verify_nematic_phase` method to calculate the nematic order parameter (S) of the weight matrix and ensure the system remains in this optimal, bio-analogous state.

### A Note on Metaphor vs. Specification

As per the Architect's Final Certification (v3.2-Sigma), certain terms used in this documentation are powerful metaphors, not literal technical specifications. Specifically, **"Infinite Hyperbolic Topology"** refers to the mathematical property of **constant negative curvature** in the learned manifold, which allows for vast and efficient embedding of hierarchical data. It does **not** imply that the system possesses infinite physical memory or size. All topological features, such as Betti numbers, are constrained by the physical hardware limits of the system.

---

## v3.3: The Tensor-Logic Architecture

Version 3.3 marks a significant evolution of the TIM VM, integrating a Tensor-Logic Kernel to create a self-auditing, hallucination-resistant reasoning system. This architecture couples a large language model (e.g., Llama-3.1-405B) with kernel-level invariant enforcement, enabling a new level of trust and stability in generative AI.

### Core Goals

-   **No Hallucinations:** By enforcing strict algebraic and geometric invariants on the model's internal representations (tensors), the system can reject computations that are likely to lead to hallucinatory or nonsensical outputs.
-   **Infinite Context:** Through techniques like Tucker decomposition of attention tensors, the system can handle arbitrarily long contexts while keeping the effective rank of the computation bounded, achieving O(1) memory per token.
-   **Self-Audit:** Every computational step is wrapped in a TIM VM transaction. If an invariant is violated, the transaction is rolled back, and a proof of the violation is logged. This creates a fully auditable and verifiable reasoning process.

### Key Components

| Component | Role | Integration points |
|-----------|------|---------------------|
| **Large Language Model (e.g., Llama-3.1)** | Provides the raw generative and embedding capabilities. | The model's embedding and attention tensors are exposed to the kernel for verification. |
| **v3.3-Tensor.ko (Tensor Logic Kernel)** | Enforces algebraic invariants (Nematic Order, Birkhoff constraints) on tensors before they are used in computation. | Registers kernel hooks (`reg_tl_verify_*`) that are called by the model's runtime. |
| **TIM VM (Tensor-Invariant-Machine)** | Provides a transactional execution environment that can roll back or reject steps that violate invariants. | Wraps each forward-pass of the model as a transaction and logs the results for auditing. |
| **LCI (Low-Level Control Interface)** | A new hardware-proximate layer (`reg_fusion_core.c`) for high-stakes environments, providing features like Triple Modular Redundancy (TMR) and cryptographic vault sealing. | Enforces hard physical constraints on the system's operation. |

### v4.8.3: CFM Resonance Integrator

Version 4.8.3 introduces the `reg_cfm_sleep.c` module, a critical component of the LCI that is responsible for tuning the Constrained Field Model (CFM) to a state of perfect resonance (`target_2t20`). This module is hardened to strictly enforce the following invariants, ensuring that all state transitions are atomic, causally ordered, and resilient to hardware faults:

-   **I38 (Causal Clock Sync):** All operations are ordered by a dedicated vector clock (`cfm_sleep_vc`) to prevent causal violations.
-   **I39 (Vault Sealer):** All state changes are wrapped in atomic transactions using the `lci_vault` API, with full rollback capabilities to guarantee system integrity.
-   **I40 (TMR ECC):** All critical state parameters (φ, ψ, and the nematic stability parameter S) are protected by Triple Modular Redundancy (TMR), ensuring that the system can withstand single-event upsets and other hardware faults without data corruption.
