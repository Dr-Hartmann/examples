def find_prime_factors(n: int) -> list[int]:
    """Возвращает список простых множителей числа n методом пробного деления."""
    factors: list[int] = []

    while n % 2 == 0:
        factors.append(2)
        n //= 2

    d = 3
    while d * d <= n:
        while n % d == 0:
            factors.append(d)
            n //= d
        d += 2

    if n > 1:
        factors.append(n)

    return factors


def main() -> None:
    n_composite = 2048  # 2**11
    result = find_prime_factors(n_composite)
    print(f"Число N: {n_composite}")
    print(f"Простые множители: {result}")


if __name__ == "__main__":
    main()
