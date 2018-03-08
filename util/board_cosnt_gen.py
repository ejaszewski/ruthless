#!/usr/bin/env python3

def bs(pos):
    x = pos // 8
    y = pos % 8
    bs = ''
    for i in range(0, 8):
        for j in range(0, 8):
            if (abs(x - i) > 0 or abs(y-j) > 0) and ((x == i or y == j) or (x - i == y - j) or (x - i == j - y)):
                bs += '1'
            else:
                bs += '0'
        # bs += '\n'
    return bs

def left(pos):
    x = pos // 8
    y = pos % 8
    bs = ''
    for i in range(0, 8):
        for j in range(0, 8):
            if y - j > 0 and x == i:
                bs += '1'
            else:
                bs += '0'
        # bs += '\n'
    return bs

def right(pos):
    x = pos // 8
    y = pos % 8
    bs = ''
    for i in range(0, 8):
        for j in range(0, 8):
            if j - y > 0 and x == i:
                bs += '1'
            else:
                bs += '0'
        # bs += '\n'
    return bs

def up(pos):
    x = pos // 8
    y = pos % 8
    bs = ''
    for i in range(0, 8):
        for j in range(0, 8):
            if x - i > 0 and y == j:
                bs += '1'
            else:
                bs += '0'
        # bs += '\n'
    return bs

def down(pos):
    x = pos // 8
    y = pos % 8
    bs = ''
    for i in range(0, 8):
        for j in range(0, 8):
            if i - x > 0 and y == j:
                bs += '1'
            else:
                bs += '0'
        # bs += '\n'
    return bs

def up_left(pos):
    x = pos // 8
    y = pos % 8
    bs = ''
    for i in range(0, 8):
        for j in range(0, 8):
            if x - i > 0 and (x - i == y - j):
                bs += '1'
            else:
                bs += '0'
        # bs += '\n'
    return bs

def up_right(pos):
    x = pos // 8
    y = pos % 8
    bs = ''
    for i in range(0, 8):
        for j in range(0, 8):
            if x - i > 0 and (x - i == j - y):
                bs += '1'
            else:
                bs += '0'
        # bs += '\n'
    return bs

def down_left(pos):
    x = pos // 8
    y = pos % 8
    bs = ''
    for i in range(0, 8):
        for j in range(0, 8):
            if i - x > 0 and (x - i == j - y):
                bs += '1'
            else:
                bs += '0'
        # bs += '\n'
    return bs

def down_right(pos):
    x = pos // 8
    y = pos % 8
    bs = ''
    for i in range(0, 8):
        for j in range(0, 8):
            if i - x > 0 and (x - i == y - j):
                bs += '1'
            else:
                bs += '0'
        # bs += '\n'
    return bs

for i in range(0, 64):
    upd = hex(int(up(i), 2))[2:]
    dwn = hex(int(down(i), 2))[2:]
    lft = hex(int(left(i), 2))[2:]
    rig = hex(int(right(i), 2))[2:]
    upl = hex(int(up_left(i), 2))[2:]
    upr = hex(int(up_right(i), 2))[2:]
    dwl = hex(int(down_left(i), 2))[2:]
    dwr = hex(int(down_right(i), 2))[2:]
    print('[%018s,%018s,%018s,%018s,%018s,%018s,%018s,%018s]' % (upd, dwn, lft, rig, upl, upr, dwl, dwr), end=',\n')
