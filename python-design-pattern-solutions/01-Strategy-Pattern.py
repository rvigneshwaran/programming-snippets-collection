from abc import ABC, abstractmethod
import cmath  # For complex number support

class DiscriminantStrategy(ABC):
    @abstractmethod
    def calculate_discriminant(self, a, b, c):
        pass


class OrdinaryDiscriminantStrategy(DiscriminantStrategy):
    def calculate_discriminant(self, a, b, c):
        return b**2 - 4*a*c


class RealDiscriminantStrategy(DiscriminantStrategy):
    def calculate_discriminant(self, a, b, c):
        discriminant = b**2 - 4*a*c
        return float('nan') if discriminant < 0 else discriminant


class QuadraticEquationSolver:
    def __init__(self, strategy):
        self.strategy = strategy

    def solve(self, a, b, c):
        """ Returns a pair of complex values """
        discriminant = self.strategy.calculate_discriminant(a, b, c)
        if isinstance(discriminant, float) and discriminant != discriminant:  # Check for NaN
            return (float('nan'), float('nan'))
        
        # Calculate the two solutions
        sqrt_discriminant = cmath.sqrt(discriminant)
        x1 = (-b + sqrt_discriminant) / (2 * a)
        x2 = (-b - sqrt_discriminant) / (2 * a)
        
        return (x1, x2)

# Example Usage
if __name__ == "__main__":
    # Test cases
    a = 1
    b = 2
    c = 1

    ordinary_solver = QuadraticEquationSolver(OrdinaryDiscriminantStrategy())
    real_solver = QuadraticEquationSolver(RealDiscriminantStrategy())

    print("Ordinary Strategy Results:", ordinary_solver.solve(a, b, c))  # Expect: (-1+0j, -1+0j)
    print("Real Strategy Results:", real_solver.solve(a, b, c))          # Expect: (-1+0j, -1+0j)

    a = 1
    b = 2
    c = 5

    print("Ordinary Strategy Results:", ordinary_solver.solve(a, b, c))  # Expect: (-1+2j, -1-2j)
    print("Real Strategy Results:", real_solver.solve(a, b, c))          # Expect: (nan, nan)
