from ..progress import padded_progress


def test_padding():
    assert padded_progress(2, 5) == "2/5"
    assert padded_progress(2, 50) == " 2/50"
    assert padded_progress(2, 500) == "  2/500"
    assert padded_progress(20, 500) == " 20/500"
    assert padded_progress(200, 500) == "200/500"
