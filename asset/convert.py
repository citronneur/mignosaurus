import struct
import sys

if __name__ == "__main__":
    raw = open(sys.argv[1], "rb")
    output = open(sys.argv[2], "w")
    
    i = 0
    output.write("pub const ASSET : &[u32] = &[")
    while True:
        try:
            value = struct.unpack("<I", raw.read(3) + b"\x00")[0]
            output.write("%s, "%(value))
        except:
            break
        
        i += 1
        
        if i % 100 == 0:
            output.write("\n")
    output.write("];")
    raw.close()
    output.close()