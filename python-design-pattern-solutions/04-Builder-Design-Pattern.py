class CodeBuilder:
    def __init__(self, class_name):
        self.class_name = class_name
        self.fields = []  # Store the fields to be added

    def add_field(self, name, value):
        # Add a field as a tuple (name, value)
        self.fields.append((name, value))
        return self  # Return self to enable method chaining

    def __str__(self):
        if not self.fields:
            # If no fields are added, return the class definition with a `pass` statement
            return f"class {self.class_name}:\n  pass"
        else:
            # Start building the class definition
            lines = [f"class {self.class_name}:", "  def __init__(self):"]
            # Add fields to the constructor
            for name, value in self.fields:
                lines.append(f"    self.{name} = {value}")
            return "\n".join(lines)

# Example usage:
foo_class_instance = CodeBuilder('Foo')
print(foo_class_instance)

# Example usage:
person_class_instance = CodeBuilder('Person').add_field('name', '""') \
                          .add_field('age', '0')
print(person_class_instance)