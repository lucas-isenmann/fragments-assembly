import random
from load_save import *

def generate_dna_fragments(dna_length, fragment_length, overlap):
    """
    Generates a random DNA sequence and creates fragments with specified overlap.

    Args:
        dna_length (int): Length of the original DNA sequence.
        fragment_length (int): Length of each fragment.
        overlap (int): Overlap between consecutive fragments.

    Returns:
        tuple: A tuple containing the original DNA sequence and a list of fragments.
    """

    # Generate the original DNA sequence
    dna_sequence = ''.join(random.choice("ACGT") for _ in range(dna_length))

    # Generate fragments
    fragments = []
    start = 0
    while start < dna_length:
        fragment = dna_sequence[start:start + fragment_length]
        if len(fragment) < fragment_length:
            break  # Stop if the fragment is shorter than desired
        fragments.append(fragment)
        start += fragment_length - overlap  # Move to the next fragment with overlap

    return dna_sequence, fragments


if __name__ == "__main__":
    name = "synth4"
    seq, fragments = generate_dna_fragments(10000, 100, 50)
    print(seq)
    print(fragments)

    with open(f"{name}.dna", "w") as f:
        f.write(seq)
    save_fragments_fasta(f"{name}.fasta", fragments)
    save_fragments_fastq(f"{name}.fastq", fragments)
    