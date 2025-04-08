
def calculate_differences(result_string, ground_truth):
    """
    Calculate various metrics to compare result string with ground truth.
    
    Args:
        result_string (str): The assembled string from the fragment assembly algorithm
        ground_truth (str): The original ground truth string
        
    Returns:
        dict: Dictionary containing various comparison metrics
    """
    # Basic statistics
    length_diff = len(result_string) - len(ground_truth)
    min_length = min(len(result_string), len(ground_truth))
    
    # Character-level comparison
    matches = sum(1 for r, g in zip(result_string[:min_length], 
                                  ground_truth[:min_length]) 
                 if r == g)
    character_accuracy = matches / max(len(result_string), len(ground_truth))
    
    # Edit distance calculation
    m, n = len(result_string), len(ground_truth)
    dp = [[0] * (n + 1) for _ in range(m + 1)]
    
    for i in range(m + 1):
        dp[i][0] = i
    for j in range(n + 1):
        dp[0][j] = j
    
    for i in range(1, m + 1):
        for j in range(1, n + 1):
            cost = 0 if result_string[i-1] == ground_truth[j-1] else 1
            dp[i][j] = min(dp[i-1][j] + 1,      # deletion
                          dp[i][j-1] + 1,        # insertion
                          dp[i-1][j-1] + cost)   # substitution
    
    edit_distance = dp[m][n]
    normalized_edit_distance = edit_distance / max(m, n)
    
    return {
        'length_difference': length_diff,
        'character_accuracy': character_accuracy,
        'edit_distance': edit_distance,
        'normalized_edit_distance': normalized_edit_distance
    }


if __name__ == "__main__":
    # Example usage
    result = "ATCGGCT"
    truth = "ATCGACT"

    metrics = calculate_differences(result, truth)

    print("Comparison Results:")
    for metric, value in metrics.items():
        print(f"{metric}: {value:.4f}")

