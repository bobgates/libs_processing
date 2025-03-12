This folder contains Rust projects to process LIBS data. I'll try to keep
stuff orgnised by documenting it in the readme file. 

(12/3/25)
The code that works now is libs_visualization. This can read in data, average 
it and produce specific graphs of the averages.

New:
libs_to_bin - this takes a collection of libs files and writes them into a
single bin file. This should make for faster access. Before this, 10-15000 
files needed to be read before the whole set was in memory. 
