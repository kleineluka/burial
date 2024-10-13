# imports
import json
import base64
import argparse
import os

# encode a string into base64
def encode_base64(text):
    """Encodes a string into base64."""
    return base64.b64encode(text.encode('utf-8')).decode('utf-8')

# decode a base64 string back to regular text
def decode_base64(text):
    """Decodes a base64 string back to regular text."""
    return base64.b64decode(text.encode('utf-8')).decode('utf-8')

# process the JSON file and encode/decode the 'pattern' field
def process_json(file_path, direction, output_path, field):
    """Processes the JSON file and encodes/decodes the 'pattern' field."""
    # open the file
    with open(file_path, 'r', encoding='utf-8') as file:
        data = json.load(file)
    # process each entry
    for entry in data:
        # translate the field
        if field in entry:
            if direction == 'to_base64':
                entry[field] = encode_base64(entry[field])
            elif direction == 'from_base64':
                entry[field] = decode_base64(entry[field])
    # save the output
    with open(output_path, 'w', encoding='utf-8') as file:
        json.dump(data, file, indent=4)
    print(f"Processing completed. Output saved to {output_path}")

# main execution..
def main():
    parser = argparse.ArgumentParser(description="Encode or decode JSON 'pattern' fields in base64.")
    parser.add_argument("file", help="Path to the input JSON file.")
    parser.add_argument("direction", choices=['to_base64', 'from_base64'], help="Direction to process: 'to_base64' or 'from_base64'.")
    parser.add_argument("output", help="Path to the output JSON file.")
    parser.add_argument("field", help="Field to encode/decode: ex. 'pattern'.")
    args = parser.parse_args()
    if not os.path.isfile(args.file):
        print(f"Error: File {args.file} does not exist.")
        return
    process_json(args.file, args.direction, args.output, args.field)

if __name__ == "__main__":
    main()
