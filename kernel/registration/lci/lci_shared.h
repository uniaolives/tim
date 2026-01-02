// kernel/registration/lci/lci_shared.h
#ifndef _LCI_SHARED_H
#define _LCI_SHARED_H

#include <linux/types.h>

// Forward declarations for opaque structures to avoid dependency loops
struct lci_vector_clock;

// Define a placeholder for the TMR memory regions
// In a real system, these would be mapped to specific hardware addresses.
extern uint64_t fusion_params_tmr[3];

#endif /* _LCI_SHARED_H */
