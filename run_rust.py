import asyncio
import my_rust_lib

async def main():
    start = time()
    x = await my_rust_lib.get_documents1()
    print(time() - start)

    start = time()
    x = await my_rust_lib.get_documents2()
    print(time() - start)

    start = time()
    x = await my_rust_lib.get_documents3()
    print(time() - start)

    start = time()
    x = await my_rust_lib.get_documents4()
    print(time() - start)




if __name__ == '__main__':
    from time import time
    for _ in range(1):
        asyncio.run(main())
