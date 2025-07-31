const print = std.debug.print;
const std = @import("std");
    
pub fn isIsogram(str: []const u8) bool {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var map = std.AutoHashMap(u8, u8).init(allocator);
    defer map.deinit(); // Deallocate the map's memory

    var has_duplicate: bool = false; 
    
    for (str) | val | {
        const normalized_val: u8 = std.ascii.toLower(val);
        switch (normalized_val) {
            32 => continue,
            45 => continue,
            else => {
              if (map.get(normalized_val)) |_| {
                has_duplicate = true;
                break;
              } else {
                map.put(normalized_val, 1) catch return false;
              }             
            }
        }
    }
    return !has_duplicate;
}
