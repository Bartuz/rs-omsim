/* Test-only fingerprints of every parsed field; no struct layout assumptions in Rust. */
#include "parse.h"
#include <stdint.h>
static void number(uint64_t *h, uint64_t n) {
    for (int i = 0; i < 8; ++i) { *h ^= n & 255; *h *= UINT64_C(1099511628211); n >>= 8; }
}
static void string(uint64_t *h, struct byte_string b) {
    number(h, b.length);
    for (size_t i = 0; i < b.length; ++i) { *h ^= b.bytes[i]; *h *= UINT64_C(1099511628211); }
}
#define N(n) number(&h, (uint64_t)(n))
#define S(s) string(&h, (s))
#define P(p) do { N((p)[0]); N((p)[1]); } while (0)
static void molecule(uint64_t *hash, struct puzzle_molecule m) {
    uint64_t h = *hash;
    N(m.number_of_atoms);
    for (uint32_t i = 0; i < m.number_of_atoms; ++i) { N(m.atoms[i].type); P(m.atoms[i].offset); }
    N(m.number_of_bonds);
    for (uint32_t i = 0; i < m.number_of_bonds; ++i) { N(m.bonds[i].type); P(m.bonds[i].from); P(m.bonds[i].to); }
    *hash = h;
}
int oracle_puzzle(const unsigned char *bytes, size_t len, uint64_t *hash) {
    struct puzzle_file *p = parse_puzzle_byte_string((struct byte_string){(unsigned char *)bytes, len});
    if (!p) return 0;
    uint64_t h = UINT64_C(14695981039346656037);
    S(p->name); N(p->creator); N(p->parts_available);
    N(p->number_of_inputs);
    for (uint32_t i = 0; i < p->number_of_inputs; ++i) molecule(&h, p->inputs[i]);
    N(p->number_of_outputs);
    for (uint32_t i = 0; i < p->number_of_outputs; ++i) molecule(&h, p->outputs[i]);
    N(p->output_scale); N(p->production_info != 0);
    if (p->production_info) {
        struct puzzle_production_info *v = p->production_info;
        N(v->shrink_left); N(v->shrink_right); N(v->isolate_inputs_from_outputs);
        N(v->number_of_cabinets);
        for (uint32_t i = 0; i < v->number_of_cabinets; ++i) { P(v->cabinets[i].position); S(v->cabinets[i].type); }
        N(v->number_of_conduits);
        for (uint32_t i = 0; i < v->number_of_conduits; ++i) {
            struct puzzle_conduit c = v->conduits[i];
            P(c.starting_position_a); P(c.starting_position_b); N(c.number_of_hexes);
            for (uint32_t j = 0; j < c.number_of_hexes; ++j) P(c.hexes[j].offset);
        }
        N(v->number_of_vials);
        for (uint32_t i = 0; i < v->number_of_vials; ++i) { P(v->vials[i].position); N(v->vials[i].style); N(v->vials[i].count); }
    }
    free_puzzle_file(p); *hash = h; return 1;
}
int oracle_solution(const unsigned char *bytes, size_t len, uint64_t *hash) {
    struct solution_file *s = parse_solution_byte_string((struct byte_string){(unsigned char *)bytes, len});
    if (!s) return 0;
    uint64_t h = UINT64_C(14695981039346656037);
    S(s->puzzle); S(s->name); N(s->solved);
    if (s->solved) { N(s->cycles); N(s->cost); N(s->area); N(s->instructions); }
    N(s->number_of_parts);
    for (uint32_t i = 0; i < s->number_of_parts; ++i) {
        struct solution_part p = s->parts[i];
        S(p.name); P(p.position); N(p.size); N(p.rotation); N(p.which_input_or_output);
        N(p.number_of_instructions);
        for (uint32_t j = 0; j < p.number_of_instructions; ++j) { N(p.instructions[j].index); N((unsigned char)p.instructions[j].instruction); }
        N(p.number_of_track_hexes);
        for (uint32_t j = 0; j < p.number_of_track_hexes; ++j) P(p.track_hexes[j].offset);
        N(p.arm_number); N(p.conduit_id); N(p.number_of_conduit_hexes);
        for (uint32_t j = 0; j < p.number_of_conduit_hexes; ++j) P(p.conduit_hexes[j].offset);
    }
    free_solution_file(s); *hash = h; return 1;
}
