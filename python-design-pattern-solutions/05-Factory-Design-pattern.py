class Person:
    def __init__(self, id, name):
        self.id = id
        self.name = name

class PersonFactory:
    def __init__(self):
        self.current_id = 0  # Initialize a counter to track the id of each person

    def create_person(self, name):
        # Create a new person with the current id and provided name
        person = Person(self.current_id, name)
        self.current_id += 1  # Increment the counter for the next person
        return person

# Example usage:
factory = PersonFactory()

person1 = factory.create_person("Alice")
person2 = factory.create_person("Bob")

print(f"Person 1: id={person1.id}, name={person1.name}")  # Output: Person 1: id=0, name=Alice
print(f"Person 2: id={person2.id}, name={person2.name}")  # Output: Person 2: id=1, name=Bob