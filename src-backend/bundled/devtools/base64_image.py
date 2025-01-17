# imports
import os
import base64
import argparse

# to base64
def encode_file_to_base64(file_path, output_dir):
    with open(file_path, 'rb') as file:
        encoded_string = base64.b64encode(file.read()).decode('utf-8')
    file_name = os.path.basename(file_path)
    file_name_without_ext = os.path.splitext(file_name)[0]
    output_file_path = os.path.join(output_dir, f"{file_name_without_ext}.base64") 
    with open(output_file_path, 'w') as output_file:
        output_file.write(encoded_string)

# loop
def encode_folder_to_base64(folder_path, output_dir):
    for root, _, files in os.walk(folder_path):
        for file in files:
            file_path = os.path.join(root, file)
            encode_file_to_base64(file_path, output_dir)

# program
def main():
    parser = argparse.ArgumentParser(description='Convert files or folders to base64.')
    parser.add_argument('input_path', help='Path to the input file or folder.')
    parser.add_argument('output_dir', help='Path to the output directory.')
    args = parser.parse_args()

    input_path = args.input_path
    output_dir = args.output_dir

    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    if os.path.isfile(input_path):
        encode_file_to_base64(input_path, output_dir)
    elif os.path.isdir(input_path):
        encode_folder_to_base64(input_path, output_dir)
    else:
        print(f"Invalid input path: {input_path}")

if __name__ == '__main__':
    main()