class Person:
    def __init__(self, age):
        self.age = age

    def drink(self):
        return 'drinking'

    def drive(self):
        return 'driving'

    def drink_and_drive(self):
        return 'driving while drunk'


class ResponsiblePerson:
    def __init__(self, person):
        self.person = person

    def drink(self):
        if self.person.age < 18:
            return "too young"
        return self.person.drink()

    def drive(self):
        if self.person.age < 16:
            return "too young"
        return self.person.drive()

    def drink_and_drive(self):
        return "dead"  # Always return "dead" if they attempt to drink and drive


# Example Usage
if __name__ == "__main__":
    # Testing various scenarios
    person1 = ResponsiblePerson(Person(20))
    print(person1.drink())  # Output: 'drinking'
    print(person1.drive())   # Output: 'driving'
    print(person1.drink_and_drive())  # Output: 'dead'

    person2 = ResponsiblePerson(Person(15))
    print(person2.drink())  # Output: 'too young'
    print(person2.drive())   # Output: 'too young'
    print(person2.drink_and_drive())  # Output: 'dead'

    person3 = ResponsiblePerson(Person(17))
    print(person3.drink())  # Output: 'drinking'
    print(person3.drive())   # Output: 'too young'
    print(person3.drink_and_drive())  # Output: 'dead'

    person4 = ResponsiblePerson(Person(16))
    print(person4.drink())  # Output: 'drinking'
    print(person4.drive())   # Output: 'driving'
    print(person4.drink_and_drive())  # Output: 'dead'