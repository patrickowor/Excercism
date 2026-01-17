const std= @import("std");
const copy = std.mem.copyForwards;

const str = []const u8;

const subjects =  [_]str{
    "malt ",
    "rat ",
    "cat ",
    "dog ",
    "cow with the crumpled horn ",
    "maiden all forlorn ",
    "man all tattered and torn ",
    "priest all shaven and shorn ",
    "rooster that crowed in the morn ",
    "farmer sowing his corn ",
    "horse and the hound and the horn ",   
};

const actions = [_]str{
    "that lay in the ",
    "that ate the ",
    "that killed the ",
    "that worried the ",
    "that tossed the ",
    "that milked the ",
    "that kissed the ",
    "that married the ",
    "that woke the ",
    "that kept the ",
    "that belonged to the ",  
};

const starter: str = "This is the ";
const ender: str = "house that Jack built.";
const newline: str = "\n";


pub fn recite(buffer: []u8, start_verse: u32, end_verse: u32) ![]const u8 {
    const start = start_verse - 1;
    var used_buffer_len: usize = 0;

    var not_first_paragraph: bool = false;
    for (start..end_verse) |end| {
        if (not_first_paragraph) {
            copy(u8, buffer[used_buffer_len..used_buffer_len + newline.len], newline[0..]);
            used_buffer_len += newline.len;
        }
        not_first_paragraph = true ;
        copy(u8, buffer[used_buffer_len..used_buffer_len + starter.len], starter[0..]);
        used_buffer_len += starter.len;
        for (0..end) |i| {
            const index = end - (i + 1);
            const action: str = actions[index];
            const subject: str = subjects[index];
            copy(u8, buffer[used_buffer_len..used_buffer_len + subject.len], subject[0..]);
            used_buffer_len += subject.len;
            copy(u8, buffer[used_buffer_len..used_buffer_len + action.len], action[0..]);
            used_buffer_len += action.len; 
        }
        copy(u8, buffer[used_buffer_len..used_buffer_len + ender.len], ender[0..]);
        used_buffer_len += ender.len; 
    }
    
    return buffer[0..used_buffer_len];
}
