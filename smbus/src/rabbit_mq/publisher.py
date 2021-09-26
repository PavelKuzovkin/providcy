"""
Descrition
"""
__author__ = 'maxdob'
__date__ = '26.09.21'
__time__ = '3:29'
__version__ = '0.1'

import pika
import settings

rmq_parameters = pika.URLParameters(settings.rmq_url_connection_str)
rmq_connection = pika.BlockingConnection(rmq_parameters)
rmq_channel = rmq_connection.channel()
rmq_channel.exchange_delete(exchange="bank_base")
rmq_channel.exchange_declare(exchange="bank_base", exchange_type='topic', durable=True)
rmq_channel.queue_bind(queue="inserer.one", exchange="bank_base", routing_key="my.inserer")
rmq_channel.queue_bind(queue="inserer.two", exchange="bank_base", routing_key="no.inserer")

rmq_channel.basic_publish(exchange='bank_base', routing_key="my.inserer", body=b"message")
