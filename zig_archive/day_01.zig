const std = @import("std");

const printAnswer = @import("./utils/print.zig").printAnswer;
const nextNumber = @import("./utils/parser.zig").nextNumber;
const Parser = std.fmt.Parser;
const ArrayList = std.ArrayList;


const split = std.mem.splitSequence;

fn part_1(input: []const u8, allocator: std.mem.Allocator) !usize {

    var answer: usize = 0;

    var left = ArrayList(usize).init(allocator);
    defer left.deinit();
    var right = ArrayList(usize).init(allocator);
    defer right.deinit();

    var num_parser = Parser{ .iter = .{.bytes = input, .i = 0}};


    while (nextNumber(&num_parser)) | left_number | {
        try left.append(left_number);
        try right.append(nextNumber(&num_parser).?);
    }
    
    std.mem.sort(usize, left.items, {}, comptime std.sort.asc(usize));
    std.mem.sort(usize, right.items, {}, comptime std.sort.asc(usize));

    for (left.items, right.items) |l, r| {
        answer += if (l > r) l - r else r - l;
    }
        
    return answer;
}
fn part_2(input: []const u8, allocator: std.mem.Allocator) !usize {

    var answer: usize = 0;

    var left = ArrayList(usize).init(allocator);
    defer left.deinit();
    var right = std.AutoHashMap(usize, usize).init(allocator);
    defer right.deinit();

    var num_parser = Parser{ .iter = .{.bytes = input, .i = 0}};
    while (nextNumber(&num_parser)) | left_number | {
        try left.append(left_number);
        const right_number = nextNumber(&num_parser).?;
        if (right.get(right_number)) |num| {
            try right.put(right_number, num + 1);
        } else {
            try right.put(right_number, 1);
        }
    }

    for (left.items) |l | {
        if (right.get(l)) | num | {
            answer += l * num;
        }
    }
        
    return answer;
}


pub fn main() !void {
    const input = @embedFile("./inputs/01.txt");

    printAnswer(1, part_1(input, std.heap.page_allocator));
    printAnswer(2, part_2(input, std.heap.page_allocator));
}

test "part 1" {
    const input = 
        \\3   4
        \\4   3
        \\2   5
        \\1   3
        \\3   9
        \\3   3
        \\
    ;

    try std.testing.expectEqual(part_1(input, std.testing.allocator), 11);
}
test "part 2" {
    const input = 
        \\3   4
        \\4   3
        \\2   5
        \\1   3
        \\3   9
        \\3   3
        \\
    ;

    try std.testing.expectEqual(part_2(input, std.testing.allocator), 31);
}
