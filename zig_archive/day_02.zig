const std = @import("std");

const printAnswer = @import("./utils/print.zig").printAnswer;
const nextNumber = @import("./utils/parser.zig").nextNumber;
const Parser = std.fmt.Parser;

fn part1(input: []const u8) !usize {
    var safe_lines: usize = 0;

    var lines = std.mem.splitSequence(u8, input, "\n");
    while (lines.next()) |line| {
        if (safeLine(line, null)) safe_lines += 1;
    }

    return safe_lines;
}

fn safeLine(line: []const u8, skip: ?usize) bool {
    if (line.len == 0) return false;

    var num_parser = Parser{ .iter = .{ .bytes = line, .i = 0 } };

    var first = nextNumber(&num_parser).?;
    if (skip == 0) first = nextNumber(&num_parser).?;

    var previous = nextNumber(&num_parser).?;
    if (skip == 1) previous = nextNumber(&num_parser).?;

    const increasing = first < previous;
    if (!safe(increasing, first, previous)) return false;

    var index: usize = 2;
    while (nextNumber(&num_parser)) |next| : (index += 1) {
        if (skip == index) {
            continue;
        }
        if (!safe(increasing, previous, next)) return false;

        previous = next;
    }

    return true;
}

fn part2(input: []const u8) !usize {
    var safe_lines: usize = 0;

    var lines = std.mem.splitSequence(u8, input, "\n");
    outer: while (lines.next()) |line| {
        if (safeLine(line, null)) {
            safe_lines += 1;
            continue;
        }
        for (0..10) |skip| {
            if (safeLine(line, skip)) {
                safe_lines += 1;
                continue :outer;
            }
        }
    }

    return safe_lines;
}

fn safe(increasing: bool, previous: usize, next: usize) bool {
    if (previous == next) {
        return false;
    } else if (increasing and (previous > next or next - previous > 3)) {
        return false;
    } else if (!increasing and (next > previous or previous - next > 3)) {
        return false;
    }

    return true;
}

pub fn main() !void {
    const input = @embedFile("./inputs/02.txt");

    printAnswer(1, part1(input));
    printAnswer(2, part2(input));
}

test "part 1" {
    const input =
        \\7 6 4 2 1
        \\1 2 7 8 9
        \\9 7 6 2 1
        \\1 3 2 4 5
        \\8 6 4 4 1
        \\1 3 6 7 9
        \\
    ;

    try std.testing.expectEqual(2, part1(input));
}

test "part 2" {
    const input =
        \\7 6 4 2 1
        \\1 2 7 8 9
        \\9 7 6 2 1
        \\1 3 2 4 5
        \\8 6 4 4 1
        \\1 3 6 7 9
        \\
    ;

    try std.testing.expectEqual(4, part2(input));
}
