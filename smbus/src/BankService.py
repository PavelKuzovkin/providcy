# -*- coding: utf-8 -*-
"""
Descrition
"""
__author__ = 'maxdob'
__date__ = '25.09.21'
__time__ = '17:07'
__version__ = '0.1'

import pika
import settings
rmq_parameters = pika.URLParameters(settings.rmq_url_connection_str)
rmq_connection = pika.BlockingConnection(rmq_parameters)
rmq_channel = rmq_connection.channel()

rmq_channel.exchange_declare(exchange='base_bank_one', exchange_type='topic', durable=True) #fanout
# rmq_channel.queue_declare(queue='154af541', durable = True)
rmq_channel.queue_bind(queue = "inserer.one" , exchange = "base_bank_one", routing_key="my.inserer")
rmq_channel.queue_bind(queue = "inserer.two" , exchange = "base_bank_one", routing_key="no.inserer")
rmq_channel.queue_bind(queue = "inserer.tree" , exchange = "base_bank_one", routing_key="no.inserer")
rmq_channel.basic_publish(exchange='base_bank_154af541', routing_key="my.inserer", body=b"init_lone")

rmq_connection.close()