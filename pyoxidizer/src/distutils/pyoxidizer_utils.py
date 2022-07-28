"""distutils.pyoxidizer_utils

This module is injected to the distribution by PyOxidizer
"""

import os, json, platform
from hashlib import sha256

try:
    pyoxidizer_state_dir = os.environ['PYOXIDIZER_DISTUTILS_STATE_DIR']
    if not os.path.isdir(pyoxidizer_state_dir):
        os.makedirs(pyoxidizer_state_dir)
except KeyError:
    raise Exception('PYOXIDIZER_DISTUTILS_STATE_DIR not defined')

def get_extension_json_path(name):
    return os.path.join(pyoxidizer_state_dir, 'extension.%s.json' % name)


def get_extension_details(name):
    try:
        with open(get_extension_json_path(name), 'r', encoding='utf-8') as fh:
            return json.load(fh)
    except FileNotFoundError:
        return None

def get_architecture():
    return ' '.join(platform.architecture())


def hash_files(files):
    hash_obj = sha256()
    files_sorted = sorted(files)
    for path in files_sorted:
        with open(path, 'rb') as fh:
            hash_obj.update(fh.read())
    return hash_obj.hexdigest()
