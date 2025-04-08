from module.olc_druijn_graphs import *
from module.load_save import *
from module.solver2 import *
from module.comparison import *




fragments = read_fragments_fasta("data/synth3.txt")
reconstructed_dna = solver2(fragments)
print("---")
print("Result:")
print(reconstructed_dna)

with open("data/synth3.dna","r") as f:


    lines = f.readlines()

    print("---")
    print(lines[0])

    print("---")
    print("diff")
    metrics = calculate_differences(reconstructed_dna, lines[0])
    print("Comparison Results:")
    for metric, value in metrics.items():
        print(f"{metric}: {value:.4f}")