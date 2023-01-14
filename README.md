# Helper development tools

This repository contains tools to help development, like code generators.

## Datasources

### fantoir-datasource

* **p31-winner** is a small Rust utility to regenerate P31_WINNERS vector,
  used by the determine_p31_winner method to pick the most relevant Wikidata
  "instance of" (P31) claim for path qualification.

  See also: [p31-winner README](datasources/fantoir/p31-winner/README.md).

## Sites

Two generators for www.nasqueron.org content from sites.json:

  * one for the mousetrap shortcuts
  * one for the question mark popup content
