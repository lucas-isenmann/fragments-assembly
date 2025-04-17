# Version 2
# with several coverage
# with proba error
# withas much C as G

import random

def generate_fragments(dna_length, p_error):
       # Generate the original DNA sequence
    dna_sequence = ''.join(random.choice("ACGT") for _ in range(dna_length))
    for _ in range(dna_length):
        
        dna_sequence.push()

    # Generate fragments
    fragments = []
    start = 0
    while start < dna_length:
        fragment = dna_sequence[start: start + fragment_length]
        fragments.append(fragment)
        start += fragment_length - overlap  # Move to the next fragment with overlap

    return dna_sequence, fragments
