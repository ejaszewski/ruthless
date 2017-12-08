while True:
    dec = int(input("Decimal:"))
    bs = bin(dec)[2:].rjust(64, '0');
    for i in range(0, 64, 8):
        print(i // 8 + 1, bs[i : i+8])
    print("  ABCDEFGH")
