import os
import glob

# Define the license header
LICENSE_HEADER = """/* Copyright 2025 Joao Guimaraes, Catp2p Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

"""

def add_license_to_file(file_path):
    # Read the file content
    with open(file_path, 'r', encoding='utf-8') as file:
        content = file.read()
    
    # Check if the license header is already present
    if content.startswith("/* Copyright"):
        print(f"License header already exists in {file_path}")
        return False
    
    # Add the license header to the content
    new_content = LICENSE_HEADER + content
    
    # Write the updated content back to the file
    with open(file_path, 'w', encoding='utf-8') as file:
        file.write(new_content)
    
    print(f"Added license header to {file_path}")
    return True

def main():
    # Get the script's directory
    script_dir = os.path.dirname(os.path.abspath(__file__))
    
    # Construct the path to the src directory
    src_dir = os.path.join(script_dir, 'catp2p', 'src')
    
    # If the script is already in the catp2p directory
    if not os.path.exists(src_dir):
        src_dir = os.path.join(script_dir, 'src')
    
    # If still not found, try the current directory
    if not os.path.exists(src_dir):
        src_dir = 'src'
    
    # Check if the src directory exists
    if not os.path.exists(src_dir):
        print(f"Error: Could not find the src directory at {src_dir}")
        return
    
    # Find all Rust files in the src directory and its subdirectories
    rust_files = glob.glob(os.path.join(src_dir, '**', '*.rs'), recursive=True)
    
    # Add license header to each file
    files_updated = 0
    for file_path in rust_files:
        if add_license_to_file(file_path):
            files_updated += 1
    
    print(f"Added license headers to {files_updated} files out of {len(rust_files)} total Rust files.")

if __name__ == "__main__":
    main()
