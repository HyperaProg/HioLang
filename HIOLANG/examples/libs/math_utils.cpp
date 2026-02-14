/*
 * HioClib Library Example - Math Utils in C++
 * Advanced mathematical functions for Hiolang
 */

#include <cmath>
#include <vector>
#include <algorithm>

/* Calculate factorial */
long long hio_factorial(int n) {
    if (n <= 1) return 1;
    return n * hio_factorial(n - 1);
}

/* Check if number is prime */
bool hio_is_prime(int num) {
    if (num < 2) return false;
    if (num == 2) return true;
    if (num % 2 == 0) return false;
    
    for (int i = 3; i * i <= num; i += 2) {
        if (num % i == 0) return false;
    }
    return true;
}

/* Get prime numbers up to n */
std::vector<int> hio_primes_up_to(int n) {
    std::vector<int> primes;
    for (int i = 2; i <= n; i++) {
        if (hio_is_prime(i)) {
            primes.push_back(i);
        }
    }
    return primes;
}

/* Calculate GCD */
int hio_gcd(int a, int b) {
    while (b != 0) {
        int temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

/* Calculate LCM */
int hio_lcm(int a, int b) {
    return (a / hio_gcd(a, b)) * b;
}

/* Power function */
double hio_power(double base, int exp) {
    return std::pow(base, exp);
}
