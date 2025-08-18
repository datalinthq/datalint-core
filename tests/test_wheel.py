#!/usr/bin/env python
"""
Smoke test for datalint-core wheel installation.
Used in CI to verify that built wheels work correctly.
"""

import sys


def test_import():
    """Test basic import of datalint_core module."""
    try:
        import datalint_core

        print("Successfully imported datalint_core")
        print(f"Module location: {datalint_core.__file__}")

        # Check for expected module attributes
        module_attrs = dir(datalint_core)
        print(f"Found {len(module_attrs)} module attributes")

        return True
    except ImportError as e:
        print(f"Failed to import datalint_core: {e}", file=sys.stderr)
        return False


def main():
    """Run smoke tests."""
    print("Running datalint-core smoke tests...")
    print("-" * 40)

    success = test_import()

    print("-" * 40)
    if success:
        print("All tests passed!")
        return 0
    else:
        print("Tests failed", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
