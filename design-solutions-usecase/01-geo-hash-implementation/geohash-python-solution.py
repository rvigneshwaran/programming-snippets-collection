from geopy.geocoders import Nominatim
import geohash2 as geohash  # Ensure to install 'geohash2' via pip if not installed

# Define a class to represent the magical creature with its coordinates
class MagicalCreature:
    def __init__(self, name, latitude, longitude):
        self.name = name
        self.latitude = latitude
        self.longitude = longitude
        self.geohash = geohash.encode(latitude, longitude, precision=5)  # Using 5-character precision for approx 5 km radius

# Create a function to find nearby creatures based on GeoHash proximity
def find_nearby_creatures(hagrid_location, creatures, precision=3):
    nearby_creatures = []
    hagrid_geohash_prefix = hagrid_location.geohash[:precision]
    for creature in creatures:
        if creature.geohash.startswith(hagrid_geohash_prefix):
            nearby_creatures.append(creature)
    return nearby_creatures

# Hagrid's current location near the Forbidden Forest
hagrid = MagicalCreature("Hagrid", 55.6433, -3.0522)  # Approximate coordinates for Hagrid

# List of magical creatures in the Forbidden Forest with approximate coordinates
creatures = [
    MagicalCreature("Hippogriff", 55.6440, -3.0505),
    MagicalCreature("Acromantula", 55.6420, -3.0535),
    MagicalCreature("Unicorn", 55.6450, -3.0580),
    MagicalCreature("Thestral", 55.6600, -3.0600),
    MagicalCreature("Bowtruckle", 55.6415, -3.0520),
]

# Find and print nearby creatures based on GeoHash prefix matching
nearby_creatures = find_nearby_creatures(hagrid, creatures)

print(f"Hagrid's GeoHash: {hagrid.geohash}")
print("Nearby creatures:")
for creature in nearby_creatures:
    print(f"{creature.name} - GeoHash: {creature.geohash} (Location: {creature.latitude}, {creature.longitude})")