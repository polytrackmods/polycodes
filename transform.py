from typing import Union
import zlib

i = [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51]

def r(e: list[int], t: int, i: int, n: int, r: bool):
    s = t // 8
    while s >= len(e):
        e.append(0)
    a = t - 8 * s
    e[s] |= n << a & 0xFF
    if a > 8 - i and not r:
        t = s + 1
        if t >= len(e):
            e.append(0)
        e[t] |= n >> 8 - a

def decode(e: str) -> Union[list[int], None]:
    t = 0
    n: list[int] = []
    s = len(e)
    for a in range(s):
        o = ord(e[a])
        if o >= len(i):
            print("yes")
            return None
        l = i[o]
        if l == -1:
            print(o)
            return None
        if 30 == (30 & l):
            r(n, t, 5, l, a == s - 1)
            t += 5
        else:
            r(n, t, 6, l, a == s - 1)
            t += 6

    return n

def decode_track_code(track_code: str):
    # Exclude "Polytrack1"
    track_code = track_code[10:]

    # zlib header 0x78DA is always encoded to "4p" and then other stuff
    # if it is not present then track code bork
    td_start = track_code.find("4p")
    if td_start == -1:
        raise ValueError("Invalid track code")

    track_data = track_code[td_start:]
    step1 = decode(track_data)
    step2 = zlib.decompress(bytes(step1))
    step3 = decode(step2.decode("utf-8"))
    step4 = zlib.decompress(bytes(step3))
    l = step4[0]
    h = step4[1 + l]
    data = step4[l + h + 2 :]

    return data


if __name__ == "__main__":
    import sys
    import hashlib

    code = decode_track_code(sys.argv[1])
    sha256_hash = hashlib.sha256()
    sha256_hash.update(code)
    print(sha256_hash.hexdigest())
