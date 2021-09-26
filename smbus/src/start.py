from service_queue import bank, insurer
import asyncio


async def timers(service, time):
    while True:
        service.update()
        await asyncio.sleep(time)

if __name__ == '__main__':

    my_insurer = insurer('strh')
    my_bank = bank('domrf')

    timeout = 60*1
    ioloop = asyncio.new_event_loop()
    asyncio.set_event_loop(ioloop)

    ioloop.run_until_complete(asyncio.gather(
        timers(my_insurer, timeout),
        timers(my_bank, timeout)
    ))
    ioloop.close()
