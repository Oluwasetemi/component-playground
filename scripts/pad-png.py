#!/usr/bin/env python3
"""Center a PNG on a transparent canvas without external Python dependencies."""

import argparse
import binascii
import struct
import zlib
from pathlib import Path

PNG_SIGNATURE = b"\x89PNG\r\n\x1a\n"


def read_chunks(data):
    if not data.startswith(PNG_SIGNATURE):
        raise ValueError("not a PNG file")
    offset = len(PNG_SIGNATURE)
    while offset < len(data):
        length = struct.unpack(">I", data[offset : offset + 4])[0]
        kind = data[offset + 4 : offset + 8]
        payload = data[offset + 8 : offset + 8 + length]
        offset += 12 + length
        yield kind, payload
        if kind == b"IEND":
            break


def paeth(a, b, c):
    p = a + b - c
    pa = abs(p - a)
    pb = abs(p - b)
    pc = abs(p - c)
    if pa <= pb and pa <= pc:
        return a
    if pb <= pc:
        return b
    return c


def decode_png(path):
    data = Path(path).read_bytes()
    width = height = bit_depth = color_type = None
    compressed = bytearray()

    for kind, payload in read_chunks(data):
        if kind == b"IHDR":
            width, height, bit_depth, color_type, compression, filter_method, interlace = struct.unpack(
                ">IIBBBBB", payload
            )
            if compression != 0 or filter_method != 0 or interlace != 0:
                raise ValueError("unsupported PNG format")
        elif kind == b"IDAT":
            compressed.extend(payload)

    if bit_depth != 8 or color_type not in (2, 6):
        raise ValueError("only 8-bit RGB/RGBA PNG files are supported")

    channels = 3 if color_type == 2 else 4
    stride = width * channels
    raw = zlib.decompress(bytes(compressed))
    rows = []
    previous = bytearray(stride)
    offset = 0

    for _ in range(height):
        filter_type = raw[offset]
        offset += 1
        row = bytearray(raw[offset : offset + stride])
        offset += stride

        for i in range(stride):
            left = row[i - channels] if i >= channels else 0
            up = previous[i]
            up_left = previous[i - channels] if i >= channels else 0
            if filter_type == 1:
                row[i] = (row[i] + left) & 0xFF
            elif filter_type == 2:
                row[i] = (row[i] + up) & 0xFF
            elif filter_type == 3:
                row[i] = (row[i] + ((left + up) // 2)) & 0xFF
            elif filter_type == 4:
                row[i] = (row[i] + paeth(left, up, up_left)) & 0xFF
            elif filter_type != 0:
                raise ValueError(f"unsupported PNG filter: {filter_type}")

        previous = row
        rows.append(row)

    rgba = bytearray(width * height * 4)
    for y, row in enumerate(rows):
        for x in range(width):
            src = x * channels
            dst = (y * width + x) * 4
            rgba[dst : dst + 3] = row[src : src + 3]
            rgba[dst + 3] = row[src + 3] if channels == 4 else 255

    return width, height, rgba


def chunk(kind, payload):
    return (
        struct.pack(">I", len(payload))
        + kind
        + payload
        + struct.pack(">I", binascii.crc32(kind + payload) & 0xFFFFFFFF)
    )


def write_png(path, width, height, rgba):
    scanlines = bytearray()
    stride = width * 4
    for y in range(height):
        scanlines.append(0)
        start = y * stride
        scanlines.extend(rgba[start : start + stride])

    ihdr = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)
    data = PNG_SIGNATURE + chunk(b"IHDR", ihdr) + chunk(b"IDAT", zlib.compress(bytes(scanlines), 9)) + chunk(b"IEND", b"")
    Path(path).write_bytes(data)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("source")
    parser.add_argument("output")
    parser.add_argument("width", type=int)
    parser.add_argument("height", type=int)
    args = parser.parse_args()

    src_w, src_h, src = decode_png(args.source)
    canvas = bytearray(args.width * args.height * 4)
    left = (args.width - src_w) // 2
    top = (args.height - src_h) // 2

    for y in range(src_h):
        dst_start = ((top + y) * args.width + left) * 4
        src_start = y * src_w * 4
        canvas[dst_start : dst_start + src_w * 4] = src[src_start : src_start + src_w * 4]

    write_png(args.output, args.width, args.height, canvas)


if __name__ == "__main__":
    main()
