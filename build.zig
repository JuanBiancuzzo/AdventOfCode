const std = @import("std");

const ArchivoCompilacion = struct {
    name: []const u8,
    path: []const u8,
};

const archivos = [_]ArchivoCompilacion {
    .{
        .name = "dia1",
        .path = "src/dia1/dia1_historian_hysteria.zig",
    },
};

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "advent_of_code",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    for (archivos) |archivo| {
        const dia = b.addStaticLibrary(.{
            .name = archivo.name,
            .root_source_file = b.path(archivo.path),
            .target = target,
            .optimize = optimize,
        });

        exe.linkLibrary(dia);
    }

    b.installArtifact(exe);

    // Ejecutando el programa
    const run_step = b.step("run", "Run the application");

    const run_exe = b.addRunArtifact(exe);
    run_step.dependOn(&run_exe.step);

    // Corriendo los tests
    const test_step = b.step("test", "Run unit tests");

    const unit_tests = b.addTest(.{
        .root_source_file = b.path("src/dia1/dia1_historian_hysteria.zig"),
        .target = b.resolveTargetQuery(.{}), // default
    });

    const run_unit_tests = b.addRunArtifact(unit_tests);
    test_step.dependOn(&run_unit_tests.step);
}
