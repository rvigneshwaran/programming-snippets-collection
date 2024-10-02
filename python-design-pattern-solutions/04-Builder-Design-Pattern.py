class CodeBuilder:
    def __init__(self, class_name):
        self.class_name = class_name
        self.fields = []  # Store the fields to be added

    def add_field(self, name, value):
        # Add a field as a tuple (name, value)
        self.fields.append((name, value))
        return self  # Return self to enable method chaining

    def __str__(self):
        # Start building the class definition
        lines = [f"class {self.class_name}:", "  def __init__(self):"]

        # Add fields to the constructor
        for name, value in self.fields:
            lines.append(f"    self.{name} = {value}")

        # Join the lines with new line characters and return the formatted string
        return "\n".join(lines)

# Example usage:
cb = CodeBuilder('Person').add_field('name', '""') \
                          .add_field('age', '0')
print(cb)