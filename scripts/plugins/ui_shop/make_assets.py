"""Draws the images of the ui_shop example into config/plugins/ui_shop/ui: four frames of a spinning coin and a star.

They are drawn here rather than taken from the game, which ships no such files and whose resources this repository does not
redistribute. Standard library only: python scripts/plugins/ui_shop/make_assets.py
"""

import math
import struct
import zlib
from pathlib import Path

OUT = Path(__file__).resolve().parents[3] / "config" / "plugins" / "ui_shop" / "ui"
SAMPLES = 4


def png(path, width, height, pixels):
    """pixels: rows of (r, g, b, a)."""
    raw = b"".join(b"\x00" + bytes(channel for pixel in row for channel in pixel) for row in pixels)

    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)

    header = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)
    path.write_bytes(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", header) + chunk(b"IDAT", zlib.compress(raw, 9)) + chunk(b"IEND", b""))


def render(width, height, shade):
    """shade(x, y) -> (r, g, b) or None, sampled SAMPLES x SAMPLES times per pixel for smooth edges."""
    rows = []
    for py in range(height):
        row = []
        for px in range(width):
            hits = []
            for sy in range(SAMPLES):
                for sx in range(SAMPLES):
                    color = shade(px + (sx + 0.5) / SAMPLES, py + (sy + 0.5) / SAMPLES)
                    if color:
                        hits.append(color)
            if not hits:
                row.append((0, 0, 0, 0))
                continue
            r, g, b = (sum(color[i] for color in hits) // len(hits) for i in range(3))
            row.append((r, g, b, 255 * len(hits) // (SAMPLES * SAMPLES)))
        rows.append(row)
    return rows


def coin(squeeze):
    size, center, radius = 24, 12.0, 10.5

    def shade(x, y):
        dx, dy = (x - center) / max(radius * squeeze, 0.8), (y - center) / radius
        distance = dx * dx + dy * dy
        if distance > 1:
            return None
        if distance > 0.72:
            return (176, 120, 20)
        light = max(0.0, 1 - ((x - center + 3) ** 2 + (y - center + 3) ** 2) / 120)
        return (min(255, 230 + int(25 * light)), min(255, 180 + int(60 * light)), 40 + int(60 * light))

    return size, render(size, size, shade)


def star():
    size, center = 48, 24.0
    points = []
    for index in range(10):
        radius = 22 if index % 2 == 0 else 9
        angle = -math.pi / 2 + index * math.pi / 5
        points.append((center + radius * math.cos(angle), center + radius * math.sin(angle)))

    def inside(x, y):
        crossings = False
        for (x1, y1), (x2, y2) in zip(points, points[1:] + points[:1]):
            if (y1 > y) != (y2 > y) and x < x1 + (y - y1) * (x2 - x1) / (y2 - y1):
                crossings = not crossings
        return crossings

    def shade(x, y):
        if not inside(x, y):
            return None
        glow = max(0.0, 1 - math.hypot(x - center, y - center) / 22)
        return (255, 190 + int(65 * glow), 40 + int(200 * glow))

    return size, render(size, size, shade)


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    for frame, squeeze in enumerate([1.0, 0.55, 0.12, 0.55]):
        size, pixels = coin(squeeze)
        png(OUT / f"coin_{frame}.png", size, size, pixels)
    size, pixels = star()
    png(OUT / "star.png", size, size, pixels)
    print(f"Wrote the ui_shop images to {OUT}")


if __name__ == "__main__":
    main()
