#!/usr/bin/env python3

import json
import numpy as np

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

mul = [3**n for n in range(0, 10)]
print(mul)

# 3^5 = 243 patterns
def extract_diag5(dark, light):
    a = xor_shift(dark, 0x0800000000000000, 59) + 2 * xor_shift(light, 0x0800000000000000, 59)
    b = xor_shift(dark, 0x0010000000000000, 52) + 2 * xor_shift(light, 0x0010000000000000, 52)
    c = xor_shift(dark, 0x0000200000000000, 45) + 2 * xor_shift(light, 0x0000200000000000, 45)
    d = xor_shift(dark, 0x0000004000000000, 38) + 2 * xor_shift(light, 0x0000004000000000, 38)
    e = xor_shift(dark, 0x0000000080000000, 31) + 2 * xor_shift(light, 0x0000000080000000, 31)
    return a * mul[0] + b * mul[1] + c * mul[2] + d * mul[3] + e * mul[4]

# 3^6 = 729 patterns
def extract_diag6(dark, light):
    a = xor_shift(dark, 0x0400000000000000, 58) + 2 * xor_shift(light, 0x0400000000000000, 58)
    b = xor_shift(dark, 0x0008000000000000, 51) + 2 * xor_shift(light, 0x0008000000000000, 51)
    c = xor_shift(dark, 0x0000100000000000, 44) + 2 * xor_shift(light, 0x0000100000000000, 44)
    d = xor_shift(dark, 0x0000002000000000, 37) + 2 * xor_shift(light, 0x0000002000000000, 37)
    e = xor_shift(dark, 0x0000000040000000, 30) + 2 * xor_shift(light, 0x0000000040000000, 30)
    f = xor_shift(dark, 0x0000000000800000, 23) + 2 * xor_shift(light, 0x0000000000800000, 23)
    return a * mul[0] + b * mul[1] + c * mul[2] + d * mul[3] + e * mul[4] + f * mul[5]

# 3^7 = 2187 patterns
def extract_diag7(dark, light):
    a = xor_shift(dark, 0x0200000000000000, 57) + 2 * xor_shift(light, 0x0200000000000000, 57)
    b = xor_shift(dark, 0x0004000000000000, 50) + 2 * xor_shift(light, 0x0004000000000000, 50)
    c = xor_shift(dark, 0x0000080000000000, 43) + 2 * xor_shift(light, 0x0000080000000000, 43)
    d = xor_shift(dark, 0x0000001000000000, 36) + 2 * xor_shift(light, 0x0000001000000000, 36)
    e = xor_shift(dark, 0x0000000020000000, 29) + 2 * xor_shift(light, 0x0000000020000000, 29)
    f = xor_shift(dark, 0x0000000000400000, 22) + 2 * xor_shift(light, 0x0000000000400000, 22)
    g = xor_shift(dark, 0x0000000000008000, 15) + 2 * xor_shift(light, 0x0000000000008000, 15)
    return a * mul[0] + b * mul[1] + c * mul[2] + d * mul[3] + e * mul[4] + f * mul[5] + g * mul[6]

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
    return a * mul[0] + b * mul[1] + c * mul[2] + d * mul[3] + e * mul[4] + f * mul[5] + g * mul[6] + h * mul[7]

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
    return a * mul[0] + b * mul[1] + c * mul[2] + d * mul[3] + e * mul[4] + f * mul[5] + g * mul[6] + h * mul[7]

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
    return a * mul[0] + b * mul[1] + c * mul[2] + d * mul[3] + e * mul[4] + f * mul[5] + g * mul[6] + h * mul[7]

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
    return a * mul[0] + b * mul[1] + c * mul[2] + d * mul[3] + e * mul[4] + f * mul[5] + g * mul[6] + h * mul[7]

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
    return a * mul[0] + b * mul[1] + c * mul[2] + d * mul[3] + e * mul[4] + f * mul[5] + g * mul[6] + h * mul[7]

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
    return a * mul[0] + b * mul[1] + c * mul[2] + d * mul[3] + e * mul[4] + f * mul[5] + g * mul[6] + h * mul[7] + i * mul[8] + j * mul[9]

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
    return a * mul[0] + b * mul[1] + c * mul[2] + d * mul[3] + e * mul[4] + f * mul[5] + g * mul[6] + h * mul[7] + i * mul[8]

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

print('Initializing starting values (zeroed).', end='...')
# diag5_weights = np.zeros(243)
# diag6_weights = np.zeros(729)
# diag7_weights = np.zeros(2187)
# diag8_weights = np.zeros(6561)
# horiz1_weights = np.zeros(6561)
# horiz2_weights = np.zeros(6561)
# horiz3_weights = np.zeros(6561)
# horiz4_weights = np.zeros(6561)
# edge2x_weights = np.zeros(59049)
# corner3x3_weights = np.zeros(19683)

diag5_weights = np.loadtxt('./trained/diag5.txt')
diag6_weights = np.loadtxt('./trained/diag6.txt')
diag7_weights = np.loadtxt('./trained/diag7.txt')
diag8_weights = np.loadtxt('./trained/diag8.txt')
horiz1_weights = np.loadtxt('./trained/horiz1.txt')
horiz2_weights = np.loadtxt('./trained/horiz1.txt')
horiz3_weights = np.loadtxt('./trained/horiz1.txt')
horiz4_weights = np.loadtxt('./trained/horiz1.txt')
edge2x_weights = np.loadtxt('./trained/edge2x.txt')
corner3x3_weights = np.loadtxt('./trained/corner3x3.txt')
print('Done.')

def get_index(tup):
    i = 0
    mul = 1
    for x in tup:
        i += x * mul
        mul *= 3
    return i

def get_indices(dark, light):
    indices = np.zeros((10,4), dtype=int)

    for i in range(0, 4):
        indices[0][i] = extract_diag5(dark, light)
        indices[1][i] = extract_diag6(dark, light)
        indices[2][i] = extract_diag7(dark, light)
        indices[3][i] = extract_diag8(dark, light)
        indices[4][i] = extract_horiz1(dark, light)
        indices[5][i] = extract_horiz2(dark, light)
        indices[6][i] = extract_horiz3(dark, light)
        indices[7][i] = extract_horiz4(dark, light)
        indices[8][i] = extract_edge2x(dark, light)
        indices[9][i] = extract_corner3x3(dark, light)

        dark = rotate_ccw(dark)
        light = rotate_ccw(light)
    return indices

def get_score(indices):
    score = 0
    for i in range(0, 4):
        score += diag5_weights[indices[0][i]]
        score += diag6_weights[indices[1][i]]
        score += diag7_weights[indices[2][i]]
        score += diag8_weights[indices[3][i]]
        score += horiz1_weights[indices[4][i]]
        score += horiz2_weights[indices[5][i]]
        score += horiz3_weights[indices[6][i]]
        score += horiz4_weights[indices[7][i]]
        score += edge2x_weights[indices[8][i]]
        score += corner3x3_weights[indices[9][i]]
    return score

def update(err, alpha, bias, indices):
    for i in range(0, 4):
        diag5_weights[indices[0][i]] -= err * alpha * (diag5_weights[indices[0][i]] + bias)
        diag6_weights[indices[1][i]] -= err * alpha * (diag6_weights[indices[1][i]] + bias)
        diag7_weights[indices[2][i]] -= err * alpha * (diag7_weights[indices[2][i]] + bias)
        diag8_weights[indices[3][i]] -= err * alpha * (diag8_weights[indices[3][i]] + bias)
        horiz1_weights[indices[4][i]] -= err * alpha * (horiz1_weights[indices[4][i]] + bias)
        horiz2_weights[indices[5][i]] -= err * alpha * (horiz2_weights[indices[5][i]] + bias)
        horiz3_weights[indices[6][i]] -= err * alpha * (horiz3_weights[indices[6][i]] + bias)
        horiz4_weights[indices[7][i]] -= err * alpha * (horiz4_weights[indices[7][i]] + bias)
        edge2x_weights[indices[8][i]] -= err * alpha * (edge2x_weights[indices[8][i]] + bias)
        corner3x3_weights[indices[9][i]] -= err * alpha * (corner3x3_weights[indices[9][i]] + bias)

print('Loading training data...')
training_data_files = []
for i in range(48, 56):
    for j in range(0, 7):
        training_data_files.append('./training/{}_random_solved_{}.json'.format(i, j))

training_data = []
num_positions = 0
for f in training_data_files:
    print('Loading data in file: {}'.format(f), end='...', flush=True)
    with open(f) as df:
        data = json.load(df)
        num_positions += data['num_positions']
        training_data += [(pos, get_indices(pos['dark_disks'], pos['light_disks'])) for pos in data['positions']]
    print(' Done.')
print('Done.')
print('Loaded {} positions.'.format(num_positions))

print('Randomizing data ordering', end='...', flush=True)
import random
random.shuffle(training_data)
print('Done.')

import time
import sys
start = time.time()

loss = 0
last_loss = 10000000000000
learnrate = 0.001
bias = 0
num_epochs = 10
epoch_size = num_positions

for i in range(0, num_epochs):
    iterstart = time.time()

    pre_epoch = [
        np.array(diag5_weights),
        np.array(diag6_weights),
        np.array(diag7_weights),
        np.array(diag8_weights),
        np.array(horiz1_weights),
        np.array(horiz2_weights),
        np.array(horiz3_weights),
        np.array(horiz4_weights),
        np.array(edge2x_weights),
        np.array(corner3x3_weights)
    ]

    print('Epoch {}, Learnrate {}:'.format(i, learnrate))
    for sample in range(0, epoch_size):
        pos, indices = training_data[sample]
        err = (get_score(indices) - pos['score'])
        update(err, learnrate, bias, indices)

        loss += abs(get_score(indices) - pos['score'])

        if sample % (epoch_size // 1000) == 0 and sample > 0:
            completion = sample / epoch_size
            progress_string = '\rProgress: [{0:50s}] {1:.1f}% {2:.1f} sec remaining'.format(
                  '=' * int(completion * 50),
                  completion * 100,
                  ((time.time() - iterstart) / completion) * (1 - completion))
            print('{:100s}'.format(progress_string), end='', flush=True)

    done_string = '\rCompleted epoch {} with avg. loss {:.1f} in {:.1f} sec.'.format(
          i, loss / epoch_size, time.time() - iterstart)
    print('{:100s}'.format(done_string))

    if loss < last_loss:
        learnrate *= 1.05
        last_loss = loss
    elif loss > last_loss:
        print('Loss increased. Reducing learn rate and reverting.')
        learnrate /= 2
        diag5_weights = pre_epoch[0]
        diag6_weights = pre_epoch[1]
        diag7_weights = pre_epoch[2]
        diag8_weights = pre_epoch[3]
        horiz1_weights = pre_epoch[4]
        horiz2_weights = pre_epoch[5]
        horiz3_weights = pre_epoch[6]
        horiz4_weights = pre_epoch[7]
        edge2x_weights = pre_epoch[8]
        corner3x3_weights = pre_epoch[9]

    loss = 0
    bias = 0

    random.shuffle(training_data)

end = time.time()

print('Ran {} epochs in: {} sec'.format(num_epochs, end - start))

print('Loading testing data...', end='...')
testing_data_files = []
for i in range(48, 56):
    for j in range(7, 8):
        testing_data_files.append('./training/{}_random_solved_{}.json'.format(i, j))

testing_data = []
num_positions = 0
for f in testing_data_files:
    with open(f) as df:
        data = json.load(df)
        num_positions += data['num_positions']
        testing_data += list(data['positions'])
print('Done.')
print('Loaded {} positions.'.format(num_positions))

import time
start = time.time()

lossavg = 0

for pos in testing_data:
    indices = get_indices(pos['dark_disks'], pos['light_disks'])
    lossavg += abs(get_score(indices) - pos['score'])

lossavg /= num_positions
print('Had average loss of {} disks over the testing set.'.format(lossavg))

np.savetxt('./trained/diag5.txt', diag5_weights)
np.savetxt('./trained/diag6.txt', diag6_weights)
np.savetxt('./trained/diag7.txt', diag7_weights)
np.savetxt('./trained/diag8.txt', diag8_weights)
np.savetxt('./trained/horiz1.txt', horiz1_weights)
np.savetxt('./trained/horiz2.txt', horiz2_weights)
np.savetxt('./trained/horiz3.txt', horiz3_weights)
np.savetxt('./trained/horiz4.txt', horiz4_weights)
np.savetxt('./trained/edge2x.txt', edge2x_weights)
np.savetxt('./trained/corner3x3.txt', corner3x3_weights)
