import module.olc_druijn_graphs as olc
from module.load_save import *
from module.solver2 import *
from module.comparison import *




fragments = read_fragments_fastq("../FA-project/sl-100K-1K.fastq")
reconstructed_dna = solver2(fragments)
print("---")
print("Markov:")
print(reconstructed_dna)

with open("./simlord.contigs.fasta", "w") as f:
    f.write(">tig\n")
    f.write(reconstructed_dna)


# with open("./data/synth3.dna","r") as f:
#     lines = f.readlines()

#     print("---")
#     print("Ground truth:")
#     print(lines[0])

#     print("---")
#     metrics = calculate_differences(reconstructed_dna, lines[0])
#     print("Markov Results:")
#     for metric, value in metrics.items():
#         print(f"{metric}: {value:.4f}")

