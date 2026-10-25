#ifndef OM_FASTFORWARD_H
#define OM_FASTFORWARD_H

#include "sim.h"

// equivalent to
//
//     while (board->cycle < cycle_limit && !board->complete && !board->collision)
//         cycle(solution, board);
//
// but faster for solutions that settle into a loop: once the state of the board at
// the start of a tape period repeats, whole periods are skipped.  pass UINT64_MAX
// for no cycle limit.  compile with -DOMSIM_NO_FAST_FORWARD to turn it off.
void run_until_complete(struct solution *solution, struct board *board, uint64_t cycle_limit);

#endif
