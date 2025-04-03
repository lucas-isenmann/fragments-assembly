from module.olc_druijn_graphs import *
from module.load_save import *




fragments = read_fragments_fasta("data/synth3.fasta")
reconstructed_dna = reconstruct_dna(fragments)
print(reconstructed_dna)
print(len(reconstructed_dna))