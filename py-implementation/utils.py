

def get_extensions_sorted_by_popularity(column_to_lines_dict):
    r = sorted(column_to_lines_dict, key=column_to_lines_dict.get)
    r.reverse()
    return r


# Excludes some extensions very unlikely to be of interest, e.g. '.lock'
def get_top_three_extensions(column_to_lines_dict):
    data = get_extensions_sorted_by_popularity(column_to_lines_dict)
    filtered_data = list(filter(lambda e: e != ".lock", data))
    return filtered_data[0:3]


def to_relative_numbers_if_enabled(args, column_to_lines):
    if not args.relative:
        return column_to_lines

    relative_column_to_lines = {}

    total_lines = float(sum(column_to_lines.values()))
    for column in list(column_to_lines.keys()):
        relative_column_to_lines[column] = round((column_to_lines[column] / total_lines) * 100, 2)

    return relative_column_to_lines


def print_file_written(filename):
    print(f"""
Wrote output to file:

    {filename}
""")
