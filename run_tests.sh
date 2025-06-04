#!/bin/sh
# Run the test suite and store results.
set -e
pytest 'quantum L.L.M.A/tests.py' -vv --junitxml=pytest_results.xml 2>&1 | tee pytest.log
