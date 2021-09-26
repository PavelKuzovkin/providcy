"""
Descrition
"""
__author__ = 'maxdob'
__date__ = '26.09.21'
__time__ = '3:29'
__version__ = '0.1'

import pika
import settings


def on_message(channel, method_frame, prop, body):
    body_str = body.decode("utf-8")
    rkey = method_frame.routing_key
    print(body_str)
    channel.basic_ack(delivery_tag=method_frame.delivery_tag)


rmq_parameters = pika.URLParameters(settings.rmq_url_connection_str)
rmq_connection = pika.BlockingConnection(rmq_parameters)
rmq_channel = rmq_connection.channel()
rmq_channel.queue_declare(queue='inserer.two', durable=True)
rmq_channel.basic_consume(on_message_callback=on_message, queue='inserer.two', auto_ack=True)
