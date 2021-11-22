import numpy as np

from pathlib import Path


#
#
# IMPORTANT - INDEXING THE BOARD AS [Z,Y,X] not as [x,y,z]
#
#

def create_starting_grid(raw):
    cells = [[ [ 0 if y == '.' else 1 for y in x.strip() ] for x in raw.splitlines() ]]
    return np.array(cells)

def create_starting_grid_b(raw):
    cells = [[[ [ 0 if y == '.' else 1 for y in x.strip() ] for x in raw.splitlines() ]]]
    return np.array(cells)

def simulation_round(d_in):
    "Step forward one round of the simulation"

    # Because we need to be able to address the cells wrapping the currently 
    # "known" space - we need to first expand d by 1 in all directions
    new_shape = (d_in.shape[0] + 2, d_in.shape[1] + 2, d_in.shape[2] + 2)
    d_prime = np.zeros(new_shape)
    # Now put the original values in starting at (1,1,1)
    d_prime[1:-1, 1:-1, 1:-1] = d_in

    d_out = np.copy(d_prime)

    for z in range(d_prime.shape[0]):
        min_z = max(0, z-1)
        max_z = min(d_prime.shape[0], z+2) # +2 because exclusive indexing
        for y in range(d_prime.shape[1]):
            min_y = max(0, y-1)
            max_y = min(d_prime.shape[1], y+2) # +2 because exclusive indexing
            for x in range(d_prime.shape[2]):
                min_x = max(0, x-1)
                max_x = min(d_prime.shape[2], x+2) # +2 because exclusive indexing
                
                d_slice = d_prime[min_z:max_z, min_y:max_y, min_x:max_x]
                activated = np.sum(d_slice) - d_prime[z,y,x]

                if d_prime[z,y,x] > 0: # currently active
                    d_out[z,y,x] = 1 if 2 <= activated <= 3 else 0
                else: # currently inactive
                    d_out[z,y,x] = 1 if activated == 3 else 0
    
    #print(d_out)
    return d_out

def simulation_round_b(d_in):
    "Step forward one round of the simulation"
    assert len(d_in.shape) == 4

    # Because we need to be able to address the cells wrapping the currently 
    # "known" space - we need to first expand d by 1 in all directions
    new_shape = (d_in.shape[0] + 2, d_in.shape[1] + 2, d_in.shape[2] + 2, d_in.shape[3] + 2)
    d_prime = np.zeros(new_shape)
    # Now put the original values in starting at (1,1,1)
    d_prime[1:-1, 1:-1, 1:-1, 1:-1] = d_in

    d_out = np.copy(d_prime)

    for w in range(d_prime.shape[0]):
        min_w = max(0, w-1)
        max_w = min(d_prime.shape[0], w+2) # +2 because exclusive indexing
        for z in range(d_prime.shape[1]):
            min_z = max(0, z-1)
            max_z = min(d_prime.shape[1], z+2) # +2 because exclusive indexing
            for y in range(d_prime.shape[2]):
                min_y = max(0, y-1)
                max_y = min(d_prime.shape[2], y+2) # +2 because exclusive indexing
                for x in range(d_prime.shape[3]):
                    min_x = max(0, x-1)
                    max_x = min(d_prime.shape[3], x+2) # +2 because exclusive indexing
                    
                    d_slice = d_prime[min_w:max_w, min_z:max_z, min_y:max_y, min_x:max_x]
                    activated = np.sum(d_slice) - d_prime[w,z,y,x]

                    if d_prime[w,z,y,x] > 0: # currently active
                        d_out[w,z,y,x] = 1 if 2 <= activated <= 3 else 0
                    else: # currently inactive
                        d_out[w,z,y,x] = 1 if activated == 3 else 0
    
    #print(d_out)
    return d_out

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_17_a.txt'
    with open(p, "rt") as f:
        raw = f.read().strip()

    # Part A
    print("===== Part A =====")
    d_n = create_starting_grid(raw)
    for _ in range(6):
        d_n = simulation_round(d_n)
        print(np.sum(d_n), "%d bytes" % (d_n.size * d_n.itemsize))

    # Part B
    print("===== Part B =====")
    d_n = create_starting_grid_b(raw)
    for _ in range(6):
        d_n = simulation_round_b(d_n)
        print(np.sum(d_n), "%d bytes" % (d_n.size * d_n.itemsize))