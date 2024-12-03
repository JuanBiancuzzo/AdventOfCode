const std = @import("std");

// pub fn parseFile();

pub fn parte1(comptime T: usize, input: *const[T]u32) u32 {
    return input[0];
}

test "pruebas_dia_1" {
    const input = [_]u32 { 11, 2,};

    const resultado = parte1(2, &input);

    try std.testing.expectEqual(11, resultado);
}
