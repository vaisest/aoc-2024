def squared(text, size):
    buf = []
    for i in range(0, size**2):
        buf.append(text[i % len(text)])
        if len(buf) == size:
            print("".join(buf))
            buf.clear()


# # You can test your function by calling it within the following block
if __name__ == "__main__":
    squared("ab", 3)
    print()
    squared("abc", 5)
    print()
    squared("aybabtu", 5)
