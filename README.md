# osm-transit-extractor [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/CanalTP/osm-transit-extractor/Continuous%20Integration?logo=github&style=flat-square)](https://github.com/CanalTP/osm-transit-extractor/actions?query=workflow%3A%22Continuous+Integration%22)

This crate is a library to extract public transport data from an [OpenStreetMap](http://www.openstreetmap.org/) file. A CSV output with several files is also provided as a quick mean to manipulate public transport data from OSM with an external tool.

The crate [osmpbfreader](https://github.com/TeXitoi/osmpbfreader-rs) is used to read the provided [OpenStreetMap PBF
files](http://wiki.openstreetmap.org/wiki/PBF_Format)

Description of the extraction process is (details here)[./documentation/README.md].

## How to use
Run the program with --help to display available parameters. The simplest way to use it is :
`osm-transit-extractor -i name_of_the_osm_file.osm.pbf`

This command will extract the public transport data and write them to CSV files in the current directory. The output directory can be changed with the use of the parameter `-o /path/to/the/dest/directory/`
