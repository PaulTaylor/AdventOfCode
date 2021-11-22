import networkx as nx
import numpy as np
import matplotlib.pyplot as plt

from collections import defaultdict
from math import sqrt
from itertools import product
from networkx.algorithms import weakly_connected_components
from pathlib import Path

def create_all_tile_orientations(tiles):
    all_tile_orientations = {}
    for raw_tid, tile in tiles.items():
        for rot in range(4):
            all_tile_orientations[(raw_tid, rot, 'none')] = np.rot90(tile, rot)
            all_tile_orientations[(raw_tid, rot, 'lr')] = np.fliplr(np.rot90(tile, rot))
            # Don't need to use flipud because the combination of flips and rotations
            # above already covers these outputs
    return all_tile_orientations

def create_edge_lookup(ato):
    edge_lookup = defaultdict(list)
    for k, tile in ato.items():
        top, left, right, bottom = get_tile_edges(tile)
        edge_lookup[top].append((*k, "top"))
        edge_lookup[left].append((*k, "left"))
        edge_lookup[right].append((*k, "right"))
        edge_lookup[bottom].append((*k, "bottom"))

    return edge_lookup

def get_next_rl_node(G, n):
    out_edges = G.out_edges(n, data=True)
    rlist = [ dst for src, dst, attr in list(out_edges) if attr['label'] == "rl" ]
    assert len(rlist) < 2
    return rlist[0] if len(rlist) == 1 else None

def get_next_bt_node(G, n):
    out_edges = G.out_edges(n, data=True)
    rlist = [ dst for src, dst, attr in list(out_edges) if attr['label'] == "bt" ]
    assert len(rlist) < 2
    return rlist[0] if len(rlist) == 1 else None

def display_image(image):
    image = np.copy(image)
    image[np.where(image == "#")] = 1
    image[np.where(image == ".")] = 0
    image[np.where(image == "O")] = 2
    plt.imshow(image.astype(int))

monster_mask = np.array(list(map(lambda x: [ True if c == "#" else False for c in x ], """                  # 
#    ##    ##    ###
 #  #  #  #  #  #   """.splitlines())))

def look_for_monsters(input_image):
    image = np.copy(input_image) # to avoid side effects

    # Look for monsters and mark them with Os where found
    monster_count = 0
    for row_idx in range(image.shape[0] - (monster_mask.shape[0] - 1)):
        for col_idx in range(image.shape[1] - (monster_mask.shape[1] - 1)):
            image_slice = image[
                (row_idx):(row_idx + monster_mask.shape[0]),
                (col_idx):(col_idx + monster_mask.shape[1])
            ]
            
            assert image_slice.shape == monster_mask.shape
            
            if all(image_slice[monster_mask] == "#"):
                image_slice[monster_mask] = "O"
                monster_count += 1

            # Now we need to copy the image_slice back into the image - as it's not actually a view!!
            image[
                (row_idx):(row_idx + monster_mask.shape[0]),
                (col_idx):(col_idx + monster_mask.shape[1])
            ] = image_slice
        
    return monster_count, np.sum(image == "#")

def create_tiles(raw):
    tiles = {}
    current = []
    current_id = None

    for line in raw.splitlines():
        if "Tile " in line:
            current_id = line[(line.index(" ") + 1):-1]
        elif len(line.strip()) == 0:
            # gap, prep for next tile
            tiles[current_id] = np.array(current, ndmin=2)
            current = []
            current_id = None
        else:
            # line in a tile
            current.append(list(line))

    tiles[current_id] = np.array(current)

    return tiles

def get_tile_edges(tile):
    ":returns: top, left, right, bottom"
    return "".join(tile[0]), \
           "".join(tile[:, 0]), \
           "".join(tile[:, -1]), \
           "".join(tile[-1])

def do_part_a(tiles):
    all_tile_orientations = create_all_tile_orientations(tiles)
    edge_lookup = create_edge_lookup(all_tile_orientations)

    G = nx.DiGraph()

    for k, edge_list in edge_lookup.items():
        for src, dst in product(edge_list, edge_list):
            if src[0] != dst[0]: # no self-loops
                if (src[-1] == "bottom" and dst[-1] == "top"):
                    G.add_edge(src[0:3], dst[0:3], label="bt")
                elif (src[-1] == "right" and dst[-1] == "left"):
                    G.add_edge(src[0:3], dst[0:3], label="rl")

    for idx, comp in enumerate(sorted(weakly_connected_components(G), 
                            key=lambda nodes: sum(x[1] for x in G.subgraph(nodes).nodes))):
        sub_G = G.subgraph(comp).copy()
        #nx.nx_agraph.write_dot(sub_G, f"./out/the-graph-{idx}.dot")

    # This will give a number of different combinations - which are all equivalent.  We'll pick the first one with the least amount of sorting in 
    sub_G = G.subgraph(min(weakly_connected_components(G), 
            key=lambda nodes: sum(x[1] for x in G.subgraph(nodes).nodes))).copy()


    ## We can now answer Part A at this point, by looking for the nodes with degree == 2
    corner_ids = [ int(n[0]) for n in sub_G.nodes if sub_G.degree(n) == 2 ]
    part_a_result = corner_ids[0] * corner_ids[1] * corner_ids[2] * corner_ids[3]

    return part_a_result, sub_G 

def do_part_b(tiles, sub_G):
    # Now we take sub_G, find the top left corner, and fill in the final image by iterating left then down
    first_tile = [ x for x in sub_G.nodes if sub_G.in_degree(x) == 0 ]
    assert len(first_tile) == 1
    first_tile_in_col = first_tile[0]

    # Reset the loop
    first_tile_in_col = first_tile[0]
    tile_shape = tiles[first_tile_in_col[0]].shape
    grid_size = int(sqrt(len(tiles)))
    image_shape = tuple(map(lambda x: x * grid_size, tile_shape))
    image = np.full(image_shape, "/")

    row_idx = 0
    while first_tile_in_col:
        this_tile_info = first_tile_in_col
        col_idx = 0
        while this_tile_info:
            #print(this_tile_info, f"\trow={row_idx}, col={col_idx}")
            this_tid = this_tile_info[0]
            
            this_tile = np.rot90(tiles[this_tid], int(this_tile_info[1]))

            if this_tile_info[2] == "lr":
                this_tile = np.fliplr(this_tile)

            #this_tile[np.where(this_tile == ".")] = 0
            #this_tile[np.where(this_tile == "#")] = 1
            #this_tile = this_tile.astype(int)

            # Now we insert the tile into the image at the appropriate co-ordinates
            image_slice = image[
                (row_idx * tile_shape[1]):((row_idx+1)*tile_shape[1]),
                (col_idx * tile_shape[0]):((col_idx+1)*tile_shape[0])
            ]
            assert this_tile.shape == image_slice.shape, (tile.shape, image_slice.shape)
            del image_slice # this was just used for a size check

            image[
                (row_idx * tile_shape[1]):((row_idx+1)*tile_shape[1]),
                (col_idx * tile_shape[0]):((col_idx+1)*tile_shape[0])
            ] = this_tile

            # Output the image for debugging - note index positions
            # plt.matshow(image).get_figure().savefig(f"out/figure-{row_idx}-{col_idx}.png")

            # Move to next row
            this_tile_info = get_next_rl_node(sub_G, this_tile_info)
            col_idx += 1

        # move to next row
        first_tile_in_col = get_next_bt_node(sub_G, first_tile_in_col)
        row_idx += 1

    # Check the tile edge duplicates are where they are supposed to be
    assert grid_size - 1 == sum(np.array_equal(image[idx, :], image[idx + 1, :]) for idx in range(0, image.shape[0] - 1))
    assert grid_size - 1 == sum(np.array_equal(image[:, idx], image[:, idx + 1]) for idx in range(0, image.shape[0] - 1))

    # Now remove the tile borders, these will be at 9,10, 19,20, etc.
    for idx in np.arange(image.shape[0], -1, -this_tile.shape[0]):
        try:
            # image[idx, :] = f"{idx/10}" # Row
            image = np.delete(image, idx, axis=0)
        except IndexError:
            pass
        try:
            # image[:, idx] = f"B{idx}"
            image = np.delete(image, idx, axis=1)
        except IndexError:
            pass
        try:
            if idx > 0:
                #image[idx - 1, :] = f"{idx/10}" # Row
                image = np.delete(image, idx-1, axis=0)
        except IndexError:
            pass
        try:
            if idx > 0:
                # image[:, idx - 1] = f"D{idx}"
                image = np.delete(image, idx-1, axis=1)
        except IndexError:
            pass

    # Need to check rotations/flips of image
    print("Now searching for monsters:")
    res = []
    for rot in range(0, 4):
        res.append(look_for_monsters(np.rot90(image, rot)))
        res.append(look_for_monsters(np.fliplr(np.rot90(image, rot))))

    max_res = max(res, key=lambda x: x[0])
    return max_res

if __name__ == "__main__":
    p = Path(__file__).parent / "input" / 'day_20_a.txt'
    with open(p, "rt") as f:
        tiles = create_tiles(f.read())

    part_a_result, sub_G = do_part_a(tiles)
    print(f"Part A answer = {part_a_result}")


    # ## Part B
    b_result, roughness = do_part_b(tiles, sub_G)
    assert b_result < 2137, "Answer is lower than this"
    assert b_result not in [1972, 2047, 2077, 2092], "Apparently this is also not the answer"
    print("There are %d monsters and roughness score is %d" % (b_result, roughness))