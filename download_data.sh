#!/bin/bash
set -e

mkdir -p inputs
cd inputs

echo "Downloading SF Bay Area data..."
wget -q -nc https://www.diag.uniroma1.it/challenge9/data/USA-road-d/USA-road-d.BAY.gr.gz
gunzip -f USA-road-d.BAY.gr.gz

echo "Downloading NY data..."
wget -q -nc https://www.diag.uniroma1.it/challenge9/data/USA-road-d/USA-road-d.NY.gr.gz
gunzip -f USA-road-d.NY.gr.gz

echo "Downloading Colorado data..."
wget -q -nc https://www.diag.uniroma1.it/challenge9/data/USA-road-d/USA-road-d.COL.gr.gz
gunzip -f USA-road-d.COL.gr.gz

echo "Downloading Florida data..."
wget -q -nc https://www.diag.uniroma1.it/challenge9/data/USA-road-d/USA-road-d.FLA.gr.gz
gunzip -f USA-road-d.FLA.gr.gz

echo "Downloading Northwest USA data..."
wget -q -nc https://www.diag.uniroma1.it/challenge9/data/USA-road-d/USA-road-d.NW.gr.gz
gunzip -f USA-road-d.NW.gr.gz

echo "Downloading Northeast USA data..."
wget -q -nc https://www.diag.uniroma1.it/challenge9/data/USA-road-d/USA-road-d.NE.gr.gz
gunzip -f USA-road-d.NE.gr.gz

echo "Done."
