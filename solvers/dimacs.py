import numpy as np
import warnings
import os

def load_dimacs(filename):
    """
    Loads a DIMACS graph file utilizing NumPy for vectorized data ingestion.
    Supports binary caching for faster loads.
    """
    bin_file = filename + ".bin"
    
    if os.path.exists(bin_file) and os.path.getmtime(bin_file) >= os.path.getmtime(filename):
        with open(bin_file, 'rb') as f:
            magic = f.read(4)
            if magic == b'FBC1':
                header = np.frombuffer(f.read(8), dtype=np.uint32)
                num_nodes, num_edges = header[0], header[1]
                
                u = np.frombuffer(f.read(num_edges * 4), dtype=np.uint32)
                v = np.frombuffer(f.read(num_edges * 4), dtype=np.uint32)
                w = np.frombuffer(f.read(num_edges * 8), dtype=np.float64)
                
                edges = np.empty((num_edges, 3), dtype=np.float64)
                edges[:, 0] = u
                edges[:, 1] = v
                edges[:, 2] = w
                
                if np.all(edges[:, 2] == np.floor(edges[:, 2])):
                    edges = edges.astype(np.int64)
                
                return int(num_nodes), edges

    with open(filename, 'rb') as f:
        num_nodes = 0
        num_edges = 0
        
        # Parse until we find the 'p' line containing metadata
        for line in f:
            if line.startswith(b'p'):
                parts = line.split()
                num_nodes = int(parts[2])
                num_edges = int(parts[3])
                break
                
        if num_nodes == 0:
            raise ValueError("Invalid DIMACS file: missing 'p' line with node count.")
            
        # Pre-allocate memory structure based on 'p' line metadata (Requirement 2)
        edges = np.empty((num_edges, 3), dtype=np.float64)
        
        # Vectorized loading of arc data (Requirements 1, 3, 5)
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            edges[:] = np.loadtxt(f, dtype=np.float64, usecols=(1, 2, 3), max_rows=num_edges, comments=b'c', ndmin=2)
            
    with open(bin_file, 'wb') as f:
        f.write(b'FBC1')
        f.write(np.array([num_nodes, num_edges], dtype=np.uint32).tobytes())
        f.write(edges[:, 0].astype(np.uint32).tobytes())
        f.write(edges[:, 1].astype(np.uint32).tobytes())
        f.write(edges[:, 2].astype(np.float64).tobytes())

    if np.all(edges[:, 2] == np.floor(edges[:, 2])):
        edges = edges.astype(np.int64)
            
    return num_nodes, edges
