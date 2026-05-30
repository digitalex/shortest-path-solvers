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

echo "Done."
