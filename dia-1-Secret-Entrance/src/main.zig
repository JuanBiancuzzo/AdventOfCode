const std = @import("std");
const Io = std.Io;

const INPUT_FILE: []const u8 = "inputs/input.txt";

const MAX_CLICK: u16 = 100;
const MAX_LINES: u16 = 4096;
const START_CLICK: u16 = 50;

const ReadParseError = error{
    OpenError,
    ReadError,
    ParseError,
    InvalidDirectionError,
    InvalidDistanceError,
    NoMemoryError,
    NoInstructionError,
};

const Direction = enum {
    LEFT,
    RIGHT,
};

const Instruction = struct {
    distance: u16,
    direction: Direction,

    fn init(direction: Direction, distance: u16) Instruction {
        return Instruction{
            .direction = direction,
            .distance = distance,
        };
    }
};

fn read_input_file(allocator: std.mem.Allocator, io: std.Io) ReadParseError![]u8 {
    const cwd = std.Io.Dir.cwd();
    const file = cwd.openFile(io, INPUT_FILE, .{ .mode = .read_only }) catch {
        return error.OpenError;
    };
    defer file.close(io);

    const length_file = file.length(io) catch {
        return error.ReadError;
    };
    if (length_file >= 5 * MAX_LINES) {
        return error.ReadError;
    }

    const buffer = allocator.alloc(u8, length_file) catch {
        return error.ReadError;
    };
    errdefer allocator.free(buffer);

    _ = file.readPositionalAll(io, buffer, 0) catch {
        return error.ReadError;
    };

    return buffer;
}

fn parse_input(allocator: std.mem.Allocator, input: []const u8) ReadParseError![]Instruction {
    var instructions: std.ArrayList(Instruction) = .empty;
    defer instructions.deinit(allocator);

    var iterator = std.mem.splitScalar(u8, input, '\n');
    while (iterator.next()) |line| {
        if (std.mem.trim(u8, line, " ").len < 1) {
            // std.debug.print("Line: {s} has less than 2 characters\n", .{line});
            // return error.InvalidLineError;
            continue;
        }

        const direction: Direction = switch (line[0]) {
            'L' => .LEFT,
            'R' => .RIGHT,
            else => {
                std.debug.print("Direction: {s} is unknown, at line of length {d}\n", .{ line, line.len });
                return error.InvalidDirectionError;
            },
        };
        const distance: u16 = std.fmt.parseInt(u16, line[1..], 10) catch {
            // std.debug.print("Distance: {s} is invalid\n", .{line[1..]});
            return error.InvalidDistanceError;
        };

        instructions.append(allocator, Instruction.init(direction, distance)) catch {
            return error.NoMemoryError;
        };
    }

    return instructions.toOwnedSlice(allocator) catch error.NoMemoryError;
}

fn secret_entrance(instructions: []const Instruction) i32 {
    var stop_at_0: i32 = 0;
    var click: u16 = START_CLICK;

    for (instructions) |instruction| {
        if (instruction.distance > MAX_CLICK) {
            continue;
        }

        switch (instruction.direction) {
            .LEFT => {
                const inverse_distance: u16 = MAX_CLICK - instruction.distance;
                click = (click + inverse_distance) % MAX_CLICK;
            },
            .RIGHT => {
                click = (click + instruction.distance) % MAX_CLICK;
            },
        }

        if (click == 0) {
            stop_at_0 += 1;
        }
    }

    return stop_at_0;
}

pub fn main(init: std.process.Init) !void {
    const arena: std.mem.Allocator = init.arena.allocator();
    const io = init.io;

    std.debug.print("--- Day 1: Secret Entrance ---\n", .{});
    // Como el allocator es la arena del init, no necesito hacer el free
    const input: []u8 = try read_input_file(arena, io);

    std.debug.print(" - Part 1:\n", .{});
    const instructions: []Instruction = try parse_input(arena, input);
    std.debug.print("\t Amount of instructions: {d}\n", .{instructions.len});
    const parte_1: i32 = secret_entrance(instructions);

    var stdout_buffer: [1024]u8 = undefined;
    var stdout_file_writer: Io.File.Writer = .init(.stdout(), io, &stdout_buffer);
    const stdout_writer = &stdout_file_writer.interface;

    try stdout_writer.print("\tResult: {d}\n", .{parte_1});
    try stdout_writer.flush(); // Don't forget to flush!
}

test "Given test - Part 1" {
    const expected_result: i32 = 3;

    const instructions = [_]Instruction{
        Instruction.init(Direction.LEFT, 68),
        Instruction.init(Direction.LEFT, 30),
        Instruction.init(Direction.RIGHT, 48),
        Instruction.init(Direction.LEFT, 5),
        Instruction.init(Direction.RIGHT, 60),
        Instruction.init(Direction.LEFT, 55),
        Instruction.init(Direction.LEFT, 1),
        Instruction.init(Direction.LEFT, 99),
        Instruction.init(Direction.RIGHT, 14),
        Instruction.init(Direction.LEFT, 82),
    };
    const result: i32 = secret_entrance(&instructions);

    try std.testing.expectEqual(expected_result, result);
}

test "Parse input - Part 1" {
    const expected_instructions = [_]Instruction{
        Instruction.init(Direction.LEFT, 68),
        Instruction.init(Direction.LEFT, 30),
        Instruction.init(Direction.RIGHT, 48),
        Instruction.init(Direction.LEFT, 5),
        Instruction.init(Direction.RIGHT, 60),
        Instruction.init(Direction.LEFT, 55),
        Instruction.init(Direction.LEFT, 1),
        Instruction.init(Direction.LEFT, 99),
        Instruction.init(Direction.RIGHT, 14),
        Instruction.init(Direction.LEFT, 82),
    };

    const allocator = std.testing.allocator;
    const input: []const u8 = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
    const instructions = try parse_input(allocator, input);
    defer allocator.free(instructions);

    try std.testing.expectEqualSlices(Instruction, &expected_instructions, instructions);
}

test "Integration - Part 1" {
    const allocator = std.testing.allocator;
    const expected_result: i32 = 1;

    const input: []const u8 =
        \\L49
        \\L24
        \\R48
        \\L16
        \\L2
        \\R8
        \\R48
        \\L27
        \\L2
        \\L47
        \\R48
        \\L41
        \\L3
        \\R39
        \\R45
        \\R43
        \\L4
        \\R47
        \\R38
        \\R17
        \\R45
        \\L61
    ;

    const instructions = try parse_input(allocator, input);
    defer allocator.free(instructions);
    const result: i32 = secret_entrance(instructions);

    try std.testing.expectEqual(expected_result, result);
}
