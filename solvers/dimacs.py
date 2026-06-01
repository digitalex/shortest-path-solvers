import numpy as np
import warnings

def load_dimacs(filename):
    """
    Loads a DIMACS graph file utilizing NumPy for vectorized data ingestion.
    """
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
        edges = np.empty((num_edges, 3), dtype=np.int32)
        
        # Vectorized loading of arc data (Requirements 1, 3, 5)
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            edges[:] = np.loadtxt(f, dtype=np.int32, usecols=(1, 2, 3), max_rows=num_edges, comments=b'c', ndmin=2)
            
        return num_nodes, edges
