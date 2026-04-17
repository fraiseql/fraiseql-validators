#!/bin/bash
# Test FIGI validation
echo "Testing FIGI: BBG000BLNNH6"
echo "Numeric expansion:"
python3 << 'PYTHON'
s = "BBG000BLNNH6"
result = ""
for c in s:
    if c.isdigit():
        result += c
    else:
        num = ord(c) - ord('A') + 10
        result += str(num // 10) + str(num % 10)
print(f"Input: {s}")
print(f"Expanded: {result}")

# Luhn check
def luhn_check(s):
    digits = [int(d) for d in s]
    sum_val = 0
    for i, d in enumerate(reversed(digits)):
        if i % 2 == 1:
            d *= 2
            if d > 9:
                d -= 9
        sum_val += d
    return sum_val % 10 == 0

print(f"Luhn valid: {luhn_check(result)}")
PYTHON
