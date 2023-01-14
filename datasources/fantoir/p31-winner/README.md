## P31_WINNERS vector generator

This tool allows to generate code for the `fantoir-datasource` repository.

When running `fantoir-datasource wikidata`, Wikidata entities are imported
as pairs of triplets for FANTOIR code and "instance of" (P31) properties.

If an entity has several claims for the P31 property, an arbitrage is needed
to pick the more relevant one, mainly to describe a pseudo-voie kind.

When a P31 property matches several values, and all are unknown,
the following message is emitted to stderr:

`Can't determine P31 winner amongst ["Q174782", "Q62685721"], Q174782 is picked.`

An update to the P31_WINNERS vector is then needed. This code will show you blocks
of values for each entity claim.

For example, for the message above:

```
        "Q174782",     // place
        "Q62685721",   // rue piétonne
```

You can then order values, the most relevant at the top,
the less relevant at bottom, and discard the not relevant ones.

That allows to update P31_WINNERS vector in `src/commands/wikidata/qualificsation.rs`.
Generate 
