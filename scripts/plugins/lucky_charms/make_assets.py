"""Draws the Lucky Ticket's inventory icon into config/plugins/lucky_charms/ui/ticket.png: a 24x24 golden ticket.

Drawn here rather than taken from the game, whose resources this repository does not redistribute. Standard library only:
python scripts/plugins/lucky_charms/make_assets.py
"""

import struct
import zlib
from pathlib import Path

OUT = Path(__file__).resolve().parents[3] / "config" / "plugins" / "lucky_charms" / "ui"
SIZE = 24


def png(path, width, height, pixels):
    """pixels: rows of (r, g, b, a)."""
    raw = b"".join(b"\x00" + bytes(channel for pixel in row for channel in pixel) for row in pixels)

    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)

    header = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)
    path.write_bytes(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", header) + chunk(b"IDAT", zlib.compress(raw, 9)) + chunk(b"IEND", b""))


def ticket(x, y):
    """A gold ticket with notched ends, a darker border and a red star in the middle."""
    if not (2 <= x <= 21 and 6 <= y <= 17):
        return (0, 0, 0, 0)
    if (x in (2, 21)) and 10 <= y <= 13:
        return (0, 0, 0, 0)
    border = x in (2, 3, 20, 21) or y in (6, 17)
    if border:
        return (150, 100, 20, 255)
    dx, dy = abs(x - 11.5), abs(y - 11.5)
    if dx + dy * 1.6 <= 3.5 or dx * 1.6 + dy <= 3.5:
        return (200, 40, 40, 255)
    if x in (7, 16) and y % 2 == 0:
        return (170, 120, 30, 255)
    return (240, 200, 70, 255)


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    pixels = [[ticket(x, y) for x in range(SIZE)] for y in range(SIZE)]
    png(OUT / "ticket.png", SIZE, SIZE, pixels)
    print(f"Wrote {OUT / 'ticket.png'}")


if __name__ == "__main__":
    main()
