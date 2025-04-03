

def print_stats(fragments):
    print("Nb fragments: ", len(fragments))
    print("Min size: ", min(len(fragment) for fragment in fragments))
    print("Max size: ", max(len(fragment) for fragment in fragments))