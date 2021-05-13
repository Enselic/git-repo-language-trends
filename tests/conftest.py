from random import randint

import pytest


@pytest.fixture
def random_output_basename(tmp_path):
    return str(tmp_path / f"output-{randint(1,100000)}")


@pytest.fixture
def tsv_output_path(random_output_basename):
    return f"{random_output_basename}.tsv"


@pytest.fixture
def csv_output_path(random_output_basename):
    return f"{random_output_basename}.csv"


@pytest.fixture
def svg_output_path(random_output_basename):
    return f"{random_output_basename}.svg"


@pytest.fixture
def png_output_path(random_output_basename):
    return f"{random_output_basename}.png"
