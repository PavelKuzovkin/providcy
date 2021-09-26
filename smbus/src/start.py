from service_queue import bank, insurer
from multiprocessing import Process

#
# my_bank.run()
#
#
# my_insurer.run()

def runner(service):
    service.run()

if __name__ == '__main__':

    my_insurer = insurer('strh')
    my_bank = bank('domrf')

    proc_i = Process(target=runner, args=(my_insurer,))
    proc_i.start()
    # proc_b = Process(target=runner, args=(my_bank,))
    # proc_b.start()
    runner(my_bank)
    proc_i.join()
