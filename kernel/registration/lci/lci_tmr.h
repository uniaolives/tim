// kernel/registration/lci/lci_tmr.h
#ifndef _LCI_TMR_H
#define _LCI_TMR_H

#include <linux/types.h>
#include <linux/math.h>

// Declare the Triple Modular Redundancy memory regions.
// This would typically be defined in a separate C file and linked.
// For the purpose of this prototype, we will define it here.
static uint64_t fusion_params_tmr[3];

/**
 * lci_tmr_write64 - Placeholder for a hardware-assisted TMR write.
 * @addr: The TMR memory address to write to.
 * @value: The 64-bit value to write.
 *
 * In a real system, this would write the value to three independent
 * memory banks.
 */
static inline void lci_tmr_write64(uint64_t *addr, double value)
{
    // This is a stub. We simulate the write by simply storing the value.
    // The double is cast to a 64-bit integer for storage.
    *addr = *((uint64_t*)&value);
}

/**
 * lci_tmr_read64 - Placeholder for a hardware-assisted TMR read with ECC.
 * @addr: The TMR memory address to read from.
 *
 * In a real system, this would read from three independent memory banks
 * and return the result of a majority vote, transparently correcting
 * any single-bit (or single-bank) error.
 *
 * Returns:
 *  The majority-voted 64-bit value.
 */
static inline double lci_tmr_read64(uint64_t *addr)
{
    // This is a stub. We simulate the read by simply returning the stored value.
    uint64_t val_int = *addr;
    return *((double*)&val_int);
}

#endif /* _LCI_TMR_H */
