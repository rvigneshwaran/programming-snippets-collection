# GeoHash Location-Based Tracking Example 🧙‍♂️🌍

This Python program demonstrates the use of the **GeoHash** algorithm in a Harry Potter-inspired use case. Imagine Hagrid, the gamekeeper at Hogwarts, wants to locate nearby magical creatures in the Forbidden Forest. By encoding geographical coordinates into GeoHashes, we can quickly find which creatures are near Hagrid based on the similarity of their GeoHashes.

## 📝 How GeoHash Works
GeoHash is a spatial indexing algorithm that encodes latitude and longitude into a string of letters and digits. It divides the Earth's surface into a grid of rectangular cells and assigns each cell a unique GeoHash. Locations with similar prefixes are close to each other, making it useful for proximity-based searches.

### Properties of GeoHash
- **Hierarchy and Precision**: Each additional character in a GeoHash string increases location precision.
- **Spatial Proximity**: Nearby locations have similar GeoHash prefixes.
- **Compact Representation**: GeoHashes are shorter than raw latitude/longitude pairs and allow efficient storage and querying.

## ⚙️ Use Case: Finding Nearby Magical Creatures
In this example, Hagrid is trying to find nearby magical creatures in the Forbidden Forest. Each creature is represented with approximate coordinates (latitude and longitude), which we convert to GeoHashes. By comparing the GeoHash prefix of each creature to Hagrid’s location, we can identify nearby creatures.

### Example Scenario
- **Hagrid’s Location**: Near the Forbidden Forest with latitude `55.6433` and longitude `-3.0522`.
- **Nearby Magical Creatures**: Unicorns, Hippogriffs, Thestrals, and others with their own coordinates.
