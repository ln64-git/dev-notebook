import sys
from utils.workflow import run_workflow

if __name__ == "__main__":
    # Get the command-line arguments
    args = sys.argv[1:]

    # Check if a prompt is provided
    if args and args[0].startswith("-prompt"):
        try:
            prompt = args[0].split("-prompt")[1].strip('"')
        except IndexError:
            prompt = None
    else:
        prompt = None

    run_workflow(prompt)