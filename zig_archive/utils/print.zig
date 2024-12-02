const std = @import("std");

pub fn printAnswer(part: u8, answer: anytype) void {
    const part_text = if (part == 1) "One" else "Two";
    std.debug.print("--- Part {s} ---\n{any}\n", .{ part_text, answer });
}
