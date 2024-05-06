
# This script materializes all models in a Quary project to TABLE
# Usage: python .hacking/project_scripts/materialize_all_to_tables.py <target_directory i.e. ../models>

import os
import yaml
def find_closest_schema_file(dir_path, base_path):
    """Search for the closest schema file in the parent directories."""
    current_path = dir_path
    while current_path != base_path:
        for file in os.listdir(current_path):
            if file.endswith("_models.yaml") or file == "schema.yaml":
                return os.path.join(current_path, file)
        parent_path = os.path.dirname(current_path)
        if parent_path == current_path:  # Reached the root directory
            break
        current_path = parent_path
    return None

def create_or_update_schema_file(base_path, dir_path, model_name):
    schema_file = find_closest_schema_file(dir_path, base_path)
    if not schema_file:
        schema_file = os.path.join(dir_path, "schema.yaml")
        with open(schema_file, 'w') as f:
            yaml.dump({'models': []}, f)
    
    with open(schema_file, 'r') as f:
        data = yaml.safe_load(f) or {'models': []}
    
    model_found = False
    for model in data['models']:
        if model['name'] == model_name:
            model['materialization'] = 'table'
            model_found = True
            break
    if not model_found:
        data['models'].append({'name': model_name, 'materialization': 'table'})
    
    with open(schema_file, 'w') as f:
        yaml.safe_dump(data, f, sort_keys=False)

def process_directory(base_path, dir_path):
    if "sources" in dir_path:
        return

    sql_files = [f for f in os.listdir(dir_path) if os.path.isfile(os.path.join(dir_path, f)) and f.endswith('.sql')]
    for sql_file in sql_files:
        model_name = os.path.splitext(sql_file)[0]
        create_or_update_schema_file(base_path, dir_path, model_name)

    for item in os.listdir(dir_path):
        full_path = os.path.join(dir_path, item)
        if os.path.isdir(full_path) and not item == "sources":
            process_directory(base_path, full_path)

def main(target_dir):
    base_path = os.path.abspath(target_dir)
    process_directory(base_path, base_path)

if __name__ == "__main__":
    import sys
    if len(sys.argv) != 2:
        print("Usage: python script.py <target_directory>")
        sys.exit(1)
    
    main(sys.argv[1])