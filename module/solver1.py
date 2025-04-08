import random
import pandas as pd
import matplotlib.pyplot as plt

from module.fragments_stats import print_stats
from module.load_save import read_fragments_fasta



def print_matrix(m):
    for line in m:
        print(line)




def longest_common_prefix_suffix(prefix, suffix):
    """
    Finds the number of common characters between the prefix and suffix.
    """
    m, n = len(prefix), len(suffix)
    count = 0
    for k in range(1, min(m, n) + 1): # TODO Optimize with decreasing k
        if prefix[:k] == suffix[-k:]: # TODO Optimize without recomputing the substrings
            count = k
    return count


if __name__ == "__main__":
    print("Unit tests longest_common_prefix_suffix ")
    print(longest_common_prefix_suffix("aabb", "aaab"))
    # Returns 3 because
    #  aabb
    # aaab
    print(longest_common_prefix_suffix("aaab", "aabb"))


def build_common_prefix_suffix_matrix(sequences):
    """
    Constructs the 2D array f where f[i][j] is the number of common characters
    between the prefix of i and the suffix of j.
    """
    n = len(sequences)
    f = [[0] * n for _ in range(n)]
    
    for i in range(n):
        for j in range(n):
            prefix = sequences[i]
            suffix = sequences[j]
            f[i][j] = longest_common_prefix_suffix(prefix, suffix)
    return f


if __name__ == "__main__":
    print("Unit test overlap matrix")
    m = build_common_prefix_suffix_matrix(["aabb", "aaab", "baab"])
    print(m[0])
    print(m[1])
    print(m[2])



def build_modified_matrix(f, t=20):
    """
    Builds f' from f such that f'[i][j] = f'[j][i] = 1 if f[i][j] >= t or f[j][i] >= t,
    otherwise f'[i][j] = f'[j][i] = 0.
    """
    n = len(f)  # Assuming f is a square matrix
    f_prime = [[0] * n for _ in range(n)]
    
    for i in range(n):
        for j in range(i, n):  # Loop through upper triangular matrix
            if f[i][j] >= t or f[j][i] >= t:
                f_prime[i][j] = 1
                f_prime[j][i] = 1  # Ensure symmetry
    
    return f_prime

if __name__ == "__main__":
    print("Unit test unoriented matrix")
    m0 = build_common_prefix_suffix_matrix(["aabb", "aaab", "baab"])
    m = build_modified_matrix(m0, 2)
    print(m[0])
    print(m[1])
    print(m[2])


def compute_mis_min_degree(f_prime):
    """
    Computes the maximal independent set of a zero-indexed graph using the minimum degree heuristic.
    """
    n = len(f_prime)  # Number of nodes
    nodes = set(range(n))  # All nodes in the graph (0 to n-1)
    independent_set = set()  # Maximal independent set

    while nodes:
        # Calculate degrees of all remaining nodes
        degrees = {node: sum(f_prime[node][neighbor] for neighbor in nodes) for node in nodes}

        # Find the node with the minimum degree
        min_degree_node = min(degrees, key=degrees.get)

        # Debug: Validate the selected node
        if min_degree_node not in nodes:
            raise ValueError(f"Invalid node {min_degree_node}. Nodes: {nodes}")

        # Add the node to the independent set
        independent_set.add(min_degree_node)

        # Remove the node and its neighbors from the graph
        neighbors = {neighbor for neighbor in nodes if f_prime[min_degree_node][neighbor] == 1}
        neighbors.add(min_degree_node)  # Include the node itself
        nodes -= neighbors  # Remove all at once
   
    return independent_set


if __name__ == "__main__":
    print("Unit test Greedy Maximal Independent Set")
    m0 = build_common_prefix_suffix_matrix(["aabb", "aaab", "baab"])
    m = build_modified_matrix(m0, 2)
    indset = compute_mis_min_degree(m)
    print(indset)




def build_h_matrix(f, mis, t=20):
    """
    Constructs the 2D matrix h[i][j] for the independent set (MIS),
    where h[i][j] is the number of linking nodes between i and j
    such that f[i][k] >= t and f[k][j] >= t.
    """
    mis = list(mis)  # Convert set to a list for indexing
    n = len(mis)
    h = [[0] * n for _ in range(n)]  # Initialize the matrix
    
    for i in range(n):
        for j in range(n):
            if i == j:
                continue  # Skip self-loops
            node_i = mis[i]
            node_j = mis[j]
            
            # Count linking nodes
            linking_nodes = 0
            for k in range(len(f)):
                if f[node_i][k] >= t and f[k][node_j] >= t:
                    linking_nodes += 1
            
            h[i][j] = linking_nodes
    
    return h




def greedy_max_weight_hamiltonian_path(h):
    """
    Constructs a greedy maximal weight Hamiltonian path for a given adjacency matrix h.
    Args:
        h: 2D list or numpy array, adjacency matrix of the graph.
    Returns:
        path: List of nodes representing the Hamiltonian path.
        weight: Total weight of the Hamiltonian path.
    """
    n = len(h)  # Number of nodes
    visited = [False] * n  # Track visited nodes
    path = []  # The Hamiltonian path
    total_weight = 0  # Total weight of the path

    # Start from the node with the highest total weight (heuristic for "centroid-like" starting point)
    current_node = max(range(n), key=lambda i: sum(h[i]))
    path.append(current_node)
    visited[current_node] = True

    first_node = current_node

    while len(path) < n:
        # Find the next unvisited node with the highest weight edge
        next_node = -1
        max_weight = -1
        insertion = "after"
        for j in range(n):
            if not visited[j] and h[current_node][j] > max_weight:
                next_node = j
                max_weight = h[current_node][j]
                insertion = "after"
            if not visited[j] and h[j][first_node] > max_weight:
                next_node = j
                max_weight = h[j][first_node]
                insertion = "before"

        # Add the node to the path and update weight
        if next_node != -1:
            if insertion == "after":
                path.append(next_node)
            else:
                path.insert(0, next_node)
            total_weight += max_weight
            visited[next_node] = True
            if insertion == "after":
                current_node = next_node
            else:
                first_node = next_node

    return path, total_weight


def clean_strings(sequences, i, j, f):
    """
    Cleans two strings by removing overlapping prefix and suffix based on the matrix f.

    Args:
        sequences (list): List of DNA fragments.
        i (int): Index of the first string.
        j (int): Index of the second string.
        f (list): Common prefix-suffix matrix.

    Returns:
        tuple: Cleaned version of the two strings.
    """
    overlap = f[i][j]
    cleaned_i = sequences[i][:-overlap] if overlap > 0 else sequences[i]
    cleaned_j = sequences[j][overlap:] if overlap > 0 else sequences[j]
    return cleaned_i, cleaned_j


def find_longest_common_substring(s1, s2):
    """
    Finds the longest common substring between two strings using dynamic programming.

    Args:
        s1 (str): First string.
        s2 (str): Second string.

    Returns:
        str: Longest common substring.
    """
    m, n = len(s1), len(s2)
    dp = [[0] * (n + 1) for _ in range(m + 1)]
    longest, end_pos = 0, 0

    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if s1[i - 1] == s2[j - 1]:
                dp[i][j] = dp[i - 1][j - 1] + 1
                if dp[i][j] > longest:
                    longest = dp[i][j]
                    end_pos = i

    return s1[end_pos - longest:end_pos]




def reconstruct_dna(fragments):
    print_stats(fragments)
    m0 = build_common_prefix_suffix_matrix(fragments)
    print_matrix(m0)
    m = build_modified_matrix(m0, 10)
    print_matrix(m)
    indset = compute_mis_min_degree(m)
    print("independent set:")
    print(indset)
    print("hmatrix")
    hm = build_h_matrix(m0, indset, 10)
    print_matrix(hm)
    path, weight = greedy_max_weight_hamiltonian_path(hm)
    print(path)
    print(weight)

    



if __name__ == "__main__":
    fragments = read_fragments_fasta("data/synth3.fasta")
    reconstruct_dna(fragments)
