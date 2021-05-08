

def get_extensions_sorted_by_popularity(column_to_lines_dict):
    r = sorted(column_to_lines_dict, key=column_to_lines_dict.get)
    r.reverse()
    return r


# Excludes some extensions very unlikely to be of interest, e.g. '.lock'
def get_top_three_extensions(column_to_lines_dict):
    l = get_extensions_sorted_by_popularity(column_to_lines_dict)
    l = list(filter(lambda e: e != ".lock", l))
    return l[0:3]
