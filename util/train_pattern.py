#!/usr/bin/env python3

import json
import numpy as np
import tensorflow as tf

def xor_shift(val, mask, shift):
    return (val & mask) >> shift

def print_board(dark, light):
    mask = 0x8000000000000000
    line = ''
    while mask > 0:
        if mask & dark > 0:
            line += '#'
        elif mask & light > 0:
            line += 'O'
        else:
            line += '_'
        mask >>= 1
        if len(line) == 8:
            print(line)
            line = ''

# 3^5 = 243 patterns
def extract_diag5(dark, light):
    a = xor_shift(dark, 0x0800000000000000, 59) + 2 * xor_shift(light, 0x0800000000000000, 59)
    b = xor_shift(dark, 0x0010000000000000, 52) + 2 * xor_shift(light, 0x0010000000000000, 52)
    c = xor_shift(dark, 0x0000200000000000, 45) + 2 * xor_shift(light, 0x0000200000000000, 45)
    d = xor_shift(dark, 0x0000004000000000, 38) + 2 * xor_shift(light, 0x0000004000000000, 38)
    e = xor_shift(dark, 0x0000000080000000, 31) + 2 * xor_shift(light, 0x0000000080000000, 31)
    return (a, b, c, d, e)

# 3^6 = 729 patterns
def extract_diag6(dark, light):
    a = xor_shift(dark, 0x0400000000000000, 58) + 2 * xor_shift(light, 0x0400000000000000, 58)
    b = xor_shift(dark, 0x0008000000000000, 51) + 2 * xor_shift(light, 0x0008000000000000, 51)
    c = xor_shift(dark, 0x0000100000000000, 44) + 2 * xor_shift(light, 0x0000100000000000, 44)
    d = xor_shift(dark, 0x0000002000000000, 37) + 2 * xor_shift(light, 0x0000002000000000, 37)
    e = xor_shift(dark, 0x0000000040000000, 30) + 2 * xor_shift(light, 0x0000000040000000, 30)
    f = xor_shift(dark, 0x0000000000800000, 23) + 2 * xor_shift(light, 0x0000000000800000, 23)
    return (a, b, c, d, e, f)

# 3^7 = 2187 patterns
def extract_diag7(dark, light):
    a = xor_shift(dark, 0x0200000000000000, 57) + 2 * xor_shift(light, 0x0200000000000000, 57)
    b = xor_shift(dark, 0x0004000000000000, 50) + 2 * xor_shift(light, 0x0004000000000000, 50)
    c = xor_shift(dark, 0x0000080000000000, 43) + 2 * xor_shift(light, 0x0000080000000000, 43)
    d = xor_shift(dark, 0x0000001000000000, 36) + 2 * xor_shift(light, 0x0000001000000000, 36)
    e = xor_shift(dark, 0x0000000020000000, 29) + 2 * xor_shift(light, 0x0000000020000000, 29)
    f = xor_shift(dark, 0x0000000000400000, 22) + 2 * xor_shift(light, 0x0000000000400000, 22)
    g = xor_shift(dark, 0x0000000000008000, 15) + 2 * xor_shift(light, 0x0000000000008000, 15)
    return (a, b, c, d, e, f, g)

# 3^8 = 6561 patterns
def extract_diag8(dark, light):
    a = xor_shift(dark, 0x0100000000000000, 56) + 2 * xor_shift(light, 0x0100000000000000, 56)
    b = xor_shift(dark, 0x0002000000000000, 49) + 2 * xor_shift(light, 0x0002000000000000, 49)
    c = xor_shift(dark, 0x0000040000000000, 42) + 2 * xor_shift(light, 0x0000040000000000, 42)
    d = xor_shift(dark, 0x0000000800000000, 35) + 2 * xor_shift(light, 0x0000000800000000, 35)
    e = xor_shift(dark, 0x0000000010000000, 28) + 2 * xor_shift(light, 0x0000000010000000, 28)
    f = xor_shift(dark, 0x0000000000200000, 21) + 2 * xor_shift(light, 0x0000000000200000, 21)
    g = xor_shift(dark, 0x0000000000004000, 14) + 2 * xor_shift(light, 0x0000000000004000, 14)
    h = xor_shift(dark, 0x0000000000000080, 7) + 2 * xor_shift(light, 0x0000000000000080, 7)
    return (a, b, c, d, e, f, g, h)

# 3^8 = 6561 patterns
def extract_horiz1(dark, light):
    a = xor_shift(dark, 0x8000000000000000, 63) + 2 * xor_shift(light, 0x8000000000000000, 63)
    b = xor_shift(dark, 0x4000000000000000, 62) + 2 * xor_shift(light, 0x4000000000000000, 62)
    c = xor_shift(dark, 0x2000000000000000, 61) + 2 * xor_shift(light, 0x2000000000000000, 61)
    d = xor_shift(dark, 0x1000000000000000, 60) + 2 * xor_shift(light, 0x1000000000000000, 60)
    e = xor_shift(dark, 0x0800000000000000, 59) + 2 * xor_shift(light, 0x0800000000000000, 59)
    f = xor_shift(dark, 0x0400000000000000, 58) + 2 * xor_shift(light, 0x0400000000000000, 58)
    g = xor_shift(dark, 0x0200000000000000, 57) + 2 * xor_shift(light, 0x0200000000000000, 57)
    h = xor_shift(dark, 0x0100000000000000, 56) + 2 * xor_shift(light, 0x0100000000000000, 56)
    return (a, b, c, d, e, f, g, h)

# 3^8 = 6561 patterns
def extract_horiz2(dark, light):
    a = xor_shift(dark, 0x80000000000000, 55) + 2 * xor_shift(light, 0x80000000000000, 55)
    b = xor_shift(dark, 0x40000000000000, 54) + 2 * xor_shift(light, 0x40000000000000, 54)
    c = xor_shift(dark, 0x20000000000000, 53) + 2 * xor_shift(light, 0x20000000000000, 53)
    d = xor_shift(dark, 0x10000000000000, 52) + 2 * xor_shift(light, 0x10000000000000, 52)
    e = xor_shift(dark, 0x08000000000000, 51) + 2 * xor_shift(light, 0x08000000000000, 51)
    f = xor_shift(dark, 0x04000000000000, 50) + 2 * xor_shift(light, 0x04000000000000, 50)
    g = xor_shift(dark, 0x02000000000000, 49) + 2 * xor_shift(light, 0x02000000000000, 49)
    h = xor_shift(dark, 0x01000000000000, 48) + 2 * xor_shift(light, 0x01000000000000, 48)
    return (a, b, c, d, e, f, g, h)

# 3^8 = 6561 patterns
def extract_horiz3(dark, light):
    a = xor_shift(dark, 0x800000000000, 47) + 2 * xor_shift(light, 0x800000000000, 47)
    b = xor_shift(dark, 0x400000000000, 46) + 2 * xor_shift(light, 0x400000000000, 46)
    c = xor_shift(dark, 0x200000000000, 45) + 2 * xor_shift(light, 0x200000000000, 45)
    d = xor_shift(dark, 0x100000000000, 44) + 2 * xor_shift(light, 0x100000000000, 44)
    e = xor_shift(dark, 0x080000000000, 43) + 2 * xor_shift(light, 0x080000000000, 43)
    f = xor_shift(dark, 0x040000000000, 42) + 2 * xor_shift(light, 0x040000000000, 42)
    g = xor_shift(dark, 0x020000000000, 41) + 2 * xor_shift(light, 0x020000000000, 41)
    h = xor_shift(dark, 0x010000000000, 40) + 2 * xor_shift(light, 0x010000000000, 40)
    return (a, b, c, d, e, f, g, h)

# 3^8 = 6561 patterns
def extract_horiz4(dark, light):
    a = xor_shift(dark, 0x8000000000, 39) + 2 * xor_shift(light, 0x8000000000, 39)
    b = xor_shift(dark, 0x4000000000, 38) + 2 * xor_shift(light, 0x4000000000, 38)
    c = xor_shift(dark, 0x2000000000, 37) + 2 * xor_shift(light, 0x2000000000, 37)
    d = xor_shift(dark, 0x1000000000, 36) + 2 * xor_shift(light, 0x1000000000, 36)
    e = xor_shift(dark, 0x0800000000, 35) + 2 * xor_shift(light, 0x0800000000, 35)
    f = xor_shift(dark, 0x0400000000, 34) + 2 * xor_shift(light, 0x0400000000, 34)
    g = xor_shift(dark, 0x0200000000, 33) + 2 * xor_shift(light, 0x0200000000, 33)
    h = xor_shift(dark, 0x0100000000, 32) + 2 * xor_shift(light, 0x0100000000, 32)
    return (a, b, c, d, e, f, g, h)

# 3^10 = 59049 patterns
def extract_edge2x(dark, light):
    a = xor_shift(dark, 0x8000000000000000, 63) + 2 * xor_shift(light, 0x8000000000000000, 63)
    b = xor_shift(dark, 0x4000000000000000, 62) + 2 * xor_shift(light, 0x4000000000000000, 62)
    c = xor_shift(dark, 0x2000000000000000, 61) + 2 * xor_shift(light, 0x2000000000000000, 61)
    d = xor_shift(dark, 0x1000000000000000, 60) + 2 * xor_shift(light, 0x1000000000000000, 60)
    e = xor_shift(dark, 0x0800000000000000, 59) + 2 * xor_shift(light, 0x0800000000000000, 59)
    f = xor_shift(dark, 0x0400000000000000, 58) + 2 * xor_shift(light, 0x0400000000000000, 58)
    g = xor_shift(dark, 0x0200000000000000, 57) + 2 * xor_shift(light, 0x0200000000000000, 57)
    h = xor_shift(dark, 0x0100000000000000, 56) + 2 * xor_shift(light, 0x0100000000000000, 56)
    i = xor_shift(dark, 0x40000000000000, 54) + 2 * xor_shift(light, 0x40000000000000, 54)
    j = xor_shift(dark, 0x02000000000000, 49) + 2 * xor_shift(light, 0x02000000000000, 49)
    return (a, b, c, d, e, f, g, h, i, j)

# 3^9 = 19683 patterns
def extract_corner3x3(dark, light):
    a = xor_shift(dark, 0x8000000000000000, 63) + 2 * xor_shift(light, 0x8000000000000000, 63)
    b = xor_shift(dark, 0x4000000000000000, 62) + 2 * xor_shift(light, 0x4000000000000000, 62)
    c = xor_shift(dark, 0x2000000000000000, 61) + 2 * xor_shift(light, 0x2000000000000000, 61)
    d = xor_shift(dark, 0x80000000000000, 55) + 2 * xor_shift(light, 0x80000000000000, 55)
    e = xor_shift(dark, 0x40000000000000, 54) + 2 * xor_shift(light, 0x40000000000000, 54)
    f = xor_shift(dark, 0x20000000000000, 53) + 2 * xor_shift(light, 0x20000000000000, 53)
    g = xor_shift(dark, 0x800000000000, 47) + 2 * xor_shift(light, 0x800000000000, 47)
    h = xor_shift(dark, 0x400000000000, 46) + 2 * xor_shift(light, 0x400000000000, 46)
    i = xor_shift(dark, 0x200000000000, 45) + 2 * xor_shift(light, 0x200000000000, 45)
    return (a, b, c, d, e, f, g, h, i)

def flip_diag_a1h8(x):
    k1 = 0x5500550055005500
    k2 = 0x3333000033330000
    k4 = 0x0f0f0f0f00000000
    t = k4 & (x ^ (x << 28))
    x ^= t ^ (t >> 28)
    t = k2 & (x ^ (x << 14))
    x ^= t ^ (t >> 14)
    t = k1 & (x ^ (x << 7))
    x ^= t ^ (t >> 7)
    return x

def flip_vertical(x):
    y = 0
    y |= x << 56 & 0xff00000000000000
    y |= x << 40 & 0x00ff000000000000
    y |= x << 24 & 0x0000ff0000000000
    y |= x <<  8 & 0x000000ff00000000
    y |= x >>  8 & 0x00000000ff000000
    y |= x >> 24 & 0x0000000000ff0000
    y |= x >> 40 & 0x000000000000ff00
    y |= x >> 56
    return y

def rotate_ccw(x):
    return flip_vertical(flip_diag_a1h8(x))

training_data = []
for i in range(48, 56):
    for j in range(0, 8):
        training_data.append('./training/{}_random_solved_{}.json'.format(i, j))

diag5_weights = np.zeros(243)
diag6_weights = np.zeros(729)
diag7_weights = np.zeros(2187)
diag8_weights = np.zeros(6561)
horiz1_weights = np.zeros(6561)
horiz2_weights = np.zeros(6561)
horiz3_weights = np.zeros(6561)
horiz4_weights = np.zeros(6561)
edge2x_weights = np.zeros(59049)
corner3x3_weights = np.zeros(19683)

def get_index(tup):
    i = 0
    mul = 1
    for x in tup:
        i += x * mul
        mul *= 3
    return i

def get_score(dark, light):
    score = 0
    for i in range(0, 4):
        score += diag5_weights[get_index(extract_diag5(dark, light))]
        score += diag6_weights[get_index(extract_diag6(dark, light))]
        score += diag7_weights[get_index(extract_diag7(dark, light))]
        score += diag8_weights[get_index(extract_diag8(dark, light))]
        score += horiz1_weights[get_index(extract_horiz1(dark, light))]
        score += horiz2_weights[get_index(extract_horiz2(dark, light))]
        score += horiz3_weights[get_index(extract_horiz3(dark, light))]
        score += horiz4_weights[get_index(extract_horiz4(dark, light))]
        score += edge2x_weights[get_index(extract_edge2x(dark, light))]
        score += corner3x3_weights[get_index(extract_corner3x3(dark, light))]

        dark = rotate_ccw(dark)
        light = rotate_ccw(light)
    return score

def update(score, expected, )
