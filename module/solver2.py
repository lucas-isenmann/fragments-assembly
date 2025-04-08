from module.solver1 import *
from module.fragments_stats import *
import random





def weight(x):
    return x*x


def solver2(fragments):
    print_stats(fragments)
    ag = build_common_prefix_suffix_matrix(fragments)
    n = len(fragments)
    print_matrix(ag)


    x = random.randint(0,n-1)
    result = fragments[x]
    visited = [False]*n
    visited[x] = True
    start = x
    end = x

    while True:
        candidates = []
        sum = 0

        for i in range(n):
            if visited[i] == False:
                if ag[i][end] > 0:
                    candidates.append((i, "out", ag[i][end]))
                    sum += weight(ag[i][end])
                if ag[start][i] > 0:
                    candidates.append((i, "in", ag[start][i]))
                    sum += weight(ag[start][i])
        
        if sum == 0:
            break
        print("---")
        print(start, end)
        # print(candidates)
        # print("sum", sum)
        r = random.randint(0, sum)
        # print("r", r)
        # print(result)
        for (i, type, w) in candidates:
            r -= weight(w)
            if r <= 0:
                print("choosed", i, type, w)
                # print(fragments[i])
                if type == "out":
                    # print(fragments[i][w:])
                    result = result + fragments[i][w:]
                    end = i 
                else:
                    # print(fragments[i][:w])
                    result = fragments[i][:w] + result
                    start = i
                visited[i] = True
                break
                
    return result




