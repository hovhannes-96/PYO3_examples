import asyncio
import my_rust_lib

async def main():
    x = await my_rust_lib.get_documents()
    for a in x:
        print(a.name)
    print(len(x))

if __name__ == '__main__':
    from time import time
    start = time()
    for _ in range(1):
        asyncio.run(main())
    print(time() - start)
