const std = @import("std");

pub fn main() !void {

    // get stdout - why so complicated?
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("@{d:5.1}", .{getInternetTime()});

    try bw.flush();
}

fn getInternetTime() f64 {
    const utcSecs = @intToFloat(f64, std.time.timestamp());
    const beats = @mod((utcSecs + @as(f64, 3600)), @as(f64, 24 * 3600)) / 86.4;
    return beats;
}
