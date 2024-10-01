from random import randint

class Generator:
    def generate(self, count):
        return [randint(1, 9) for _ in range(count)]

class Splitter:
    def split(self, array):
        result = []

        row_count = len(array)
        col_count = len(array[0])

        for r in range(row_count):
            result.append(array[r])  # Rows

        for c in range(col_count):
            the_col = []
            for r in range(row_count):
                the_col.append(array[r][c])
            result.append(the_col)  # Columns

        diag1 = []
        diag2 = []

        for r in range(row_count):
            diag1.append(array[r][r])          # Main diagonal
            diag2.append(array[r][row_count - r - 1])  # Anti diagonal

        result.append(diag1)  # Main diagonal
        result.append(diag2)  # Anti diagonal

        return result

class Verifier:
    def verify(self, arrays):
        first = sum(arrays[0])
        for i in range(1, len(arrays)):
            if sum(arrays[i]) != first:
                return False
        return True

class MagicSquareGenerator:
    def generate(self, size):
        while True:
            # Generate a list of numbers for a size x size matrix
            count = size * size
            numbers = Generator().generate(count)

            # Convert to a 2D matrix
            magic_square = [numbers[i * size:(i + 1) * size] for i in range(size)]

            # Split into rows, columns, and diagonals
            splitter = Splitter()
            splits = splitter.split(magic_square)

            # Verify if it's a magic square
            verifier = Verifier()
            if verifier.verify(splits):
                return magic_square

# Example usage
if __name__ == "__main__":
    magic_square_gen = MagicSquareGenerator()
    magic_square = magic_square_gen.generate(3)  # Generate a 3x3 magic square
    for row in magic_square:
        print(row)