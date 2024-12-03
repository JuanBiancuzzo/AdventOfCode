const std = @import("std");
const dia1 = @import("dia1/dia1_historian_hysteria.zig");

pub fn main() !void {
    std.debug.print("Hola tanto tiempo\n", .{});

    _ = dia1.parte1();
}
