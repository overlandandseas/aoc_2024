const std = @import("std");

pub fn nextNumber(parser: *std.fmt.Parser) ?usize {
    while (parser.peek(0)) |c| {
        _ = switch(c) {
            '0'...'9' => return parser.number(),
            else => parser.iter.nextCodepoint(),
        };
    } else {
        return null;
    }
}
