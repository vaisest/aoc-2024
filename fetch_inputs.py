import requests
from pathlib import Path

api_key = ""
try:
    with open("./api_key.txt", "r") as f:
        api_key = f.read()
except IOError:
    api_key = input(
        "please insert aoc cookie containing the session key (excluding the 'session=' and ';'):"
    )
    with open("./api_key.txt", "w") as f:
        f.write(api_key)


def write_day(day, data):
    file = Path(f"input/day_{day:02}.txt")
    if not file.exists():
        with open(file, "w") as f:
            f.write(data)
    else:
        print(f"{file} already exists")


for day in range(1, 25 + 1):
    print(f"Fetching day {day}")
    cookies = {"session": api_key}
    req = requests.get(
        f"https://adventofcode.com/2024/day/{day}/input", cookies=cookies
    )
    if req.status_code == 404:
        print(f"Day {day} not available yet")
        continue
    req.raise_for_status()

    write_day(day, req.text)
