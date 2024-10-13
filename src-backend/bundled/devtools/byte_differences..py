# imports
import sys

# essentially our main
def compare_and_generate_diff(old_file, new_file, diff_output):
    # open the files in binary mode
    with open(old_file, 'rb') as old_f, open(new_file, 'rb') as new_f:
        old_bytes = old_f.read()
        new_bytes = new_f.read()
    # initialize a list to hold differences
    differences = []
    # determine the minimum length to compare
    min_length = min(len(old_bytes), len(new_bytes))
    # compare byte by byte
    for i in range(min_length):
        if old_bytes[i] != new_bytes[i]:
            # record the position and new byte value
            differences.append((i, new_bytes[i]))
    # handle cases where the new file is longer
    if len(new_bytes) > len(old_bytes):
        for i in range(len(old_bytes), len(new_bytes)):
            differences.append((i, new_bytes[i]))
    # write the differences to the output file
    output_file = f"{diff_output}\\diff_output.txt"
    with open(output_file, 'w') as diff_f:
        for i, (pos, byte) in enumerate(differences):
            if i < len(differences) - 1:
                diff_f.write(f"{pos}:{byte:02x},")
            else:
                diff_f.write(f"{pos}:{byte:02x}")
    print(f"Found {len(differences)} differences. Output written to {output_file}.")

# main execution, format: python script.py old_file new_file output_directory
if len(sys.argv) != 4:
    print("Usage: python byte_differences.py <old_file> <new_file> <output_directory>")
else:
    old_file_path = sys.argv[1]
    new_file_path = sys.argv[2]
    output_directory = sys.argv[3]
    compare_and_generate_diff(old_file_path, new_file_path, output_directory)
