import os

def dump(src_directory, output_file):
    with open(output_file, 'w') as outfile:
        for filename in os.listdir(src_directory):
            file_path = os.path.join(src_directory, filename)
            if os.path.isfile(file_path):
                with open(file_path, 'r') as infile:
                    content = infile.read()
                    outfile.write(f"FILE: {filename}\n")
                    outfile.write(content)
                    outfile.write("\n\n")

src_dir = './'
output_file = 'dump.txt'

dump(src_dir, output_file)
