



def read_fragments_fasta(file_path):
    """
    FASTA format
    
    >id1
    ATGGTGAAG
    >id2
    GAGGAAGGACG

    returns
    ["ATGG...", "GAGG..."]
    """
    sequences = []
    with open(file_path, 'r') as file:
        current_sequence = ""
        for line in file:
            line = line.strip()
            if line.startswith(">"):
                if current_sequence:
                    sequences.append(current_sequence)
                    current_sequence = ""
            else:
                current_sequence += line
        if current_sequence:
            sequences.append(current_sequence)
    return sequences



def save_fragments_fasta(file_path, fragments):
    with open(file_path, "w") as f:
        for fragment in fragments:
            f.write(f"> {len(fragment)}\n")
            f.write(fragment + "\n")



def save_fragments_fastq(file_path, fragments):
    with open(file_path, 'w') as f:
        for i, seq in enumerate(fragments):
            f.write(f"@sequence_{i+1}\n")
            f.write(f"{seq}\n")
            f.write("+\n")  # Quality scores placeholder
            f.write("!" * len(seq) + "\n")  # Default quality scores