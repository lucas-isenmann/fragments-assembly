import random
import matplotlib.pyplot as plt
import pandas as pd

def generate_dna_sequence(length):
    return ''.join(random.choice("ACGT") for _ in range(length))

def create_fragments(dna_sequence, fragment_length, overlap):
    fragments = []
    i = 0
    while i < len(dna_sequence):
        fragment = dna_sequence[i:i + fragment_length]
        if len(fragment) < fragment_length:
            break
        fragments.append(fragment)
        i += fragment_length - overlap
    return fragments

def calculate_overlap(a, b):
    max_overlap = 0
    for i in range(1, min(len(a), len(b)) + 1):
        if a[-i:] == b[:i]:
            max_overlap = i
    return max_overlap

def build_overlap_graph(fragments):
    graph = {}
    for i, frag_a in enumerate(fragments):
        max_overlap = 0
        best_match = None
        for j, frag_b in enumerate(fragments):
            if i != j:
                overlap = calculate_overlap(frag_a, frag_b)
                if overlap > max_overlap:
                    max_overlap = overlap
                    best_match = frag_b
        if best_match:
            graph[frag_a] = (best_match, max_overlap)
    return graph

def construct_consensus_olc(fragments, overlap_graph):
    candidates = set(overlap_graph.keys())
    for _, (target, _) in overlap_graph.items():
        if target in candidates:
            candidates.remove(target)
    start_fragment = candidates.pop() if candidates else list(overlap_graph.keys())[0]

    consensus = start_fragment
    current_fragment = start_fragment
    visited = set()
    visited.add(current_fragment)

    while current_fragment in overlap_graph:
        next_fragment, overlap = overlap_graph[current_fragment]
        if next_fragment in visited:
            break
        consensus += next_fragment[overlap:]
        visited.add(next_fragment)
        current_fragment = next_fragment

    return consensus

def build_de_bruijn_graph(kmers):
    from collections import defaultdict
    graph = defaultdict(list)
    for kmer in kmers:
        prefix = kmer[:-1]
        suffix = kmer[1:]
        graph[prefix].append(suffix)
    return graph

def find_eulerian_path(graph):
    from collections import defaultdict, deque

    in_degree = defaultdict(int)
    out_degree = defaultdict(int)
    for node, neighbors in graph.items():
        out_degree[node] += len(neighbors)
        for neighbor in neighbors:
            in_degree[neighbor] += 1

    start_node = None
    for node in graph.keys():
        if out_degree[node] > in_degree[node]:
            start_node = node
            break

    stack = [start_node]
    path = []
    while stack:
        current = stack[-1]
        if graph[current]:
            next_node = graph[current].pop()
            stack.append(next_node)
        else:
            path.append(stack.pop())
    return path[::-1]




def reconstruct_dna(fragments):
    """
    returns the computed DNA seqence
    """
    graph = build_de_bruijn_graph(fragments)
    
    path = find_eulerian_path(graph)
    
    dna_sequence = path[0]
    for i in range(1, len(path)):
        dna_sequence += path[i][-1]
    
    return dna_sequence